//! Live interactive preview system for the example viewer.
//!
//! Provides the `LivePreview` trait and wrapper implementations for all examples,
//! enabling live interactive demos in the viewer's right panel.

// The prelude imports blaeck::Box which shadows std::boxed::Box.
// We need explicit aliasing to use both.
use std::boxed::Box as StdBox;

use blaeck::input::Key;
use blaeck::prelude::*;
use crossterm::event::KeyCode;

// ============================================================================
// LivePreview Trait
// ============================================================================

pub trait LivePreview {
    /// Render the current state as an Element tree.
    fn render(&mut self) -> Element;

    /// Handle a key event. Return true to exit focus mode.
    fn handle_key(&mut self, key: &Key) -> bool {
        let _ = key;
        false
    }

    /// Called every frame with delta time in seconds.
    fn tick(&mut self, dt: f64) {
        let _ = dt;
    }

    /// Whether this preview needs continuous ticking (animations).
    /// If false, only re-renders on key input.
    fn needs_tick(&self) -> bool {
        false
    }

    /// Drain any async/background events.
    fn poll_events(&mut self) {}
}

// ============================================================================
// Static Wrapper — just re-calls build_ui()
// ============================================================================

struct StaticLive {
    build_fn: fn() -> Element,
}

impl LivePreview for StaticLive {
    fn render(&mut self) -> Element {
        (self.build_fn)()
    }
}

// ============================================================================
// Timer-Animated Wrapper — wraps build_ui_with_timer examples
// ============================================================================

struct TimerLive {
    timer: AnimationTimer,
    render_fn: fn(&AnimationTimer) -> Element,
}

impl LivePreview for TimerLive {
    fn render(&mut self) -> Element {
        (self.render_fn)(&self.timer)
    }

    fn needs_tick(&self) -> bool {
        true
    }

    fn tick(&mut self, _dt: f64) {
        // AnimationTimer auto-advances from its internal Instant
    }
}

// ============================================================================
// Interactive Wrappers
// ============================================================================

// --- interactive.rs ---
struct InteractiveLive {
    state: super::interactive::AppState,
}

impl LivePreview for InteractiveLive {
    fn render(&mut self) -> Element {
        self.state.render()
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        self.state.handle_input(key);
        false
    }
}

// --- focus_demo.rs ---
struct FocusDemoLive {
    state: super::focus_demo::AppState,
}

impl LivePreview for FocusDemoLive {
    fn render(&mut self) -> Element {
        super::focus_demo::render(&self.state)
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        self.state.handle_key(key)
    }
}

// --- form_demo.rs ---
struct FormDemoLive {
    state: super::form_demo::FormState,
}

impl LivePreview for FormDemoLive {
    fn render(&mut self) -> Element {
        super::form_demo::render(&self.state)
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        self.state.handle_key(key)
    }
}

// --- select_demo.rs ---
struct SelectDemoLive {
    state: super::select_demo::AppState,
}

impl LivePreview for SelectDemoLive {
    fn render(&mut self) -> Element {
        super::select_demo::render(&self.state)
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        self.state.handle_key(key)
    }
}

// ============================================================================
// Component-State Wrappers
// ============================================================================

// --- menu.rs ---
use blaeck::{SelectItem, SelectState};

struct MenuLive {
    items: Vec<SelectItem>,
    state: SelectState,
}

impl LivePreview for MenuLive {
    fn render(&mut self) -> Element {
        super::menu::build_ui_with_state(&self.items, &self.state)
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key.code {
            KeyCode::Up => self.state.up(),
            KeyCode::Down => self.state.down(),
            KeyCode::Char('q') => return true,
            _ => {}
        }
        false
    }
}

// --- tabs.rs ---
use blaeck::TabsState;

struct TabsLive {
    state: TabsState,
}

impl LivePreview for TabsLive {
    fn render(&mut self) -> Element {
        super::tabs::build_ui_with_state(super::tabs::TABS, super::tabs::CONTENTS, &self.state)
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key.code {
            KeyCode::Left => self.state.prev(),
            KeyCode::Right => self.state.next(),
            KeyCode::Char('q') => return true,
            _ => {}
        }
        false
    }
}

// --- multiselect.rs ---
use blaeck::MultiSelectState;

struct MultiSelectLive {
    state: MultiSelectState,
}

impl LivePreview for MultiSelectLive {
    fn render(&mut self) -> Element {
        super::multiselect::build_ui_with_state(super::multiselect::ITEMS, &self.state)
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key.code {
            KeyCode::Up => self.state.up(),
            KeyCode::Down => self.state.down(),
            KeyCode::Char(' ') => self.state.toggle(),
            KeyCode::Char('a') => self.state.toggle_all(),
            KeyCode::Char('q') => return true,
            _ => {}
        }
        false
    }
}

// --- autocomplete.rs ---
use blaeck::AutocompleteState;

struct AutocompleteLive {
    state: AutocompleteState,
}

impl AutocompleteLive {
    fn update_filtered_count(&mut self) {
        let suggestions = super::autocomplete::SUGGESTIONS;
        let count = if self.state.input.is_empty() {
            suggestions.len().min(6)
        } else {
            let input_lower = self.state.input.to_lowercase();
            suggestions
                .iter()
                .filter(|s| s.to_lowercase().contains(&input_lower))
                .take(6)
                .count()
        };
        self.state.set_filtered_count(count);
    }
}

impl LivePreview for AutocompleteLive {
    fn render(&mut self) -> Element {
        super::autocomplete::build_ui_with_state(super::autocomplete::SUGGESTIONS, &self.state)
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key.code {
            KeyCode::Up => self.state.prev(),
            KeyCode::Down => self.state.next(),
            KeyCode::Backspace => {
                self.state.backspace();
                self.update_filtered_count();
            }
            KeyCode::Char(c) => {
                self.state.insert(c);
                self.update_filtered_count();
            }
            KeyCode::Left => {
                self.state.move_left();
            }
            KeyCode::Right => {
                self.state.move_right();
            }
            KeyCode::Enter | KeyCode::Tab => {
                let suggestions = super::autocomplete::SUGGESTIONS;
                let props = AutocompleteProps::new(suggestions.to_vec())
                    .input(&self.state.input)
                    .selected(self.state.selected)
                    .filter_mode(FilterMode::Contains);
                if let Some(value) = props.selected_value() {
                    self.state.set_input(value);
                    self.update_filtered_count();
                }
            }
            _ => {}
        }
        false
    }
}

// --- polish_demo.rs ---
use blaeck::ConfirmProps;

struct PolishDemoLive {
    confirm: ConfirmProps,
}

impl LivePreview for PolishDemoLive {
    fn render(&mut self) -> Element {
        super::polish_demo::build_ui_with_confirm(&self.confirm)
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key.code {
            KeyCode::Left | KeyCode::Right => self.confirm.toggle(),
            KeyCode::Char('q') => return true,
            _ => {}
        }
        false
    }
}

// ============================================================================
// Sparkline Wrapper — randomizes data on tick
// ============================================================================

struct SparklineLive {
    cpu_data: Vec<f64>,
    mem_data: Vec<f64>,
    net_data: Vec<f64>,
    audio_data: Vec<f64>,
    seed: u64,
    elapsed: f64,
}

impl SparklineLive {
    fn pseudo_random(&mut self) -> f64 {
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.seed >> 33) as f64 / u32::MAX as f64
    }
}

impl LivePreview for SparklineLive {
    fn render(&mut self) -> Element {
        let cpu_current = *self.cpu_data.last().unwrap_or(&50.0);
        let mem_current = *self.mem_data.last().unwrap_or(&50.0);
        let net_current = *self.net_data.last().unwrap_or(&30.0);
        super::sparkline::build_ui_with_data(
            &self.cpu_data,
            cpu_current,
            &self.mem_data,
            mem_current,
            &self.net_data,
            net_current,
            &self.audio_data,
        )
    }

    fn needs_tick(&self) -> bool {
        true
    }

    fn tick(&mut self, dt: f64) {
        self.elapsed += dt;
        // Update every ~200ms
        if self.elapsed < 0.2 {
            return;
        }
        self.elapsed = 0.0;

        let cpu_v = (self.pseudo_random() * 30.0 + 35.0).clamp(0.0, 100.0);
        self.cpu_data.push(cpu_v);
        if self.cpu_data.len() > 20 {
            self.cpu_data.remove(0);
        }

        let mem_v = (self.pseudo_random() * 10.0 + 55.0).clamp(0.0, 100.0);
        self.mem_data.push(mem_v);
        if self.mem_data.len() > 20 {
            self.mem_data.remove(0);
        }

        let net_v = (self.pseudo_random() * 60.0 + 10.0).clamp(0.0, 100.0);
        self.net_data.push(net_v);
        if self.net_data.len() > 20 {
            self.net_data.remove(0);
        }

        let audio_v = (self.pseudo_random() * 80.0 + 10.0).clamp(0.0, 100.0);
        self.audio_data.push(audio_v);
        if self.audio_data.len() > 24 {
            self.audio_data.remove(0);
        }
    }
}

// ============================================================================
// Stateful Animated Wrappers
// ============================================================================

// --- dashboard.rs ---
struct DashboardLive {
    state: super::dashboard::DashboardState,
    params: super::dashboard::FieldParams,
    last_render_ms: f32,
}

impl LivePreview for DashboardLive {
    fn render(&mut self) -> Element {
        super::dashboard::build_dashboard(&self.state, &self.params)
    }

    fn needs_tick(&self) -> bool {
        true
    }

    fn tick(&mut self, dt: f64) {
        let render_start = std::time::Instant::now();
        // Get element tree stats from last frame (approximate)
        let node_count = self.state.layout.nodes;
        let tree_depth = self.state.layout.depth;
        let field_energy =
            super::dashboard::panel_field_energy(0.5, 0.5, self.state.field_time, &self.params);
        self.state.update(
            dt,
            self.last_render_ms,
            field_energy,
            node_count,
            tree_depth,
        );
        self.last_render_ms = render_start.elapsed().as_secs_f32() * 1000.0;
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key.code {
            KeyCode::Char(' ') => self.state.paused = !self.state.paused,
            KeyCode::Char('r') => self.state.restart_boot(),
            KeyCode::Char('q') => return true,
            _ => {}
        }
        false
    }
}

// --- plasma.rs ---
struct PlasmaLive {
    params: super::plasma::Params,
    lava: super::plasma::LavaLamp,
    time: f64,
}

impl LivePreview for PlasmaLive {
    fn render(&mut self) -> Element {
        let display = super::plasma::build_display(60, 20, self.time, &self.params, &self.lava);
        let info = super::plasma::build_info(&self.params);
        element! {
            Box(flex_direction: FlexDirection::Column) {
                Box(flex_direction: FlexDirection::Row) {
                    Box(border_style: BorderStyle::Round, border_color: Color::Rgb(100, 60, 160)) {
                        #(display)
                    }
                    #(info)
                }
                Text(content: "m:lava  t/T:theme  p/P:preset  r:random  +/-:speed  q:quit", dim: true)
            }
        }
    }

    fn needs_tick(&self) -> bool {
        true
    }

    fn tick(&mut self, dt: f64) {
        self.time += dt * self.params.speed;
        if self.params.mode == super::plasma::Mode::LavaLamp {
            self.lava.update(dt, self.params.speed);
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key.code {
            KeyCode::Char('t') => self.params.next_theme(),
            KeyCode::Char('T') => self.params.prev_theme(),
            KeyCode::Char('p') => self.params.next_preset(),
            KeyCode::Char('P') => self.params.prev_preset(),
            KeyCode::Char('r') => self.params.randomize_plasma(),
            KeyCode::Char('m') => {
                self.params.mode = match self.params.mode {
                    super::plasma::Mode::Plasma => super::plasma::Mode::LavaLamp,
                    super::plasma::Mode::LavaLamp => super::plasma::Mode::Plasma,
                };
            }
            KeyCode::Char('+') | KeyCode::Char('=') => {
                self.params.speed = (self.params.speed + 0.1).min(5.0)
            }
            KeyCode::Char('-') => self.params.speed = (self.params.speed - 0.1).max(0.1),
            KeyCode::Char('q') => return true,
            _ => {}
        }
        false
    }
}

// --- showcase.rs ---
struct ShowcaseLive {
    state: super::showcase::ShowcaseState,
}

impl LivePreview for ShowcaseLive {
    fn render(&mut self) -> Element {
        super::showcase::build_showcase(&self.state)
    }

    fn needs_tick(&self) -> bool {
        true
    }

    fn tick(&mut self, dt: f64) {
        self.state.update(dt, 0.0);
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key.code {
            KeyCode::Char(' ') => self.state.toggle_pause(),
            KeyCode::Char('t') => self.state.next_theme(),
            KeyCode::Char('r') => self.state.restart(),
            KeyCode::Char('q') => return true,
            _ => {}
        }
        false
    }
}

// --- cube3d_braille.rs ---
struct Cube3dLive {
    state: super::cube3d_braille::AppState,
}

impl LivePreview for Cube3dLive {
    fn render(&mut self) -> Element {
        super::cube3d_braille::build_ui_with_state(&self.state)
    }

    fn needs_tick(&self) -> bool {
        true
    }

    fn tick(&mut self, dt: f64) {
        if self.state.auto_rotate {
            self.state.angle_x += dt as f32 * 0.8;
            self.state.angle_y += dt as f32 * 1.1;
            self.state.angle_z += dt as f32 * 0.3;
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key.code {
            KeyCode::Char(' ') => self.state.auto_rotate = !self.state.auto_rotate,
            KeyCode::Char('s') => self.state.next_shape(),
            KeyCode::Char('c') => self.state.next_color(),
            KeyCode::Up => self.state.angle_x -= 0.1,
            KeyCode::Down => self.state.angle_x += 0.1,
            KeyCode::Left => self.state.angle_y -= 0.1,
            KeyCode::Right => self.state.angle_y += 0.1,
            KeyCode::Char('q') => return true,
            _ => {}
        }
        false
    }
}

// ============================================================================
// Async Wrapper — background worker via std::thread + mpsc
// ============================================================================

use std::sync::mpsc;

struct AsyncAppLive {
    state: super::async_app::AppState,
    receiver: mpsc::Receiver<super::async_app::BackgroundEvent>,
    event_tx: mpsc::Sender<super::async_app::BackgroundEvent>,
}

impl AsyncAppLive {
    fn start_background_work(&mut self) {
        if self.state.is_loading {
            return;
        }
        let input = self.state.input.clone();
        self.state.input.clear();
        self.state.is_loading = true;
        self.state.progress = 0;
        self.state.spinner_start = std::time::Instant::now();
        self.state.status = "Processing...".to_string();

        let tx = self.event_tx.clone();
        std::thread::spawn(move || {
            for i in 0..=100 {
                std::thread::sleep(std::time::Duration::from_millis(20));
                let _ = tx.send(super::async_app::BackgroundEvent::Progress(i));
            }
            let _ = tx.send(super::async_app::BackgroundEvent::DataChunk(format!(
                "Processed: \"{}\"\n",
                input
            )));
            let _ = tx.send(super::async_app::BackgroundEvent::Done);
        });
    }
}

impl LivePreview for AsyncAppLive {
    fn render(&mut self) -> Element {
        super::async_app::render(&self.state)
    }

    fn needs_tick(&self) -> bool {
        true
    }

    fn poll_events(&mut self) {
        while let Ok(event) = self.receiver.try_recv() {
            super::async_app::handle_background_event(&mut self.state, event);
        }
    }

    fn tick(&mut self, _dt: f64) {
        // spinner animation is driven by Instant internally
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key.code {
            KeyCode::Enter => {
                if !self.state.is_loading && !self.state.input.is_empty() {
                    self.start_background_work();
                }
            }
            KeyCode::Backspace => {
                self.state.input.pop();
            }
            KeyCode::Char(c) => {
                if !self.state.is_loading {
                    self.state.input.push(c);
                }
            }
            _ => {}
        }
        false
    }
}

// ============================================================================
// Reactive Wrapper — embeds RuntimeHandle manually
// ============================================================================

use blaeck::reactive::{ComponentId, RuntimeHandle, Scope};

struct ReactivePreviewLive {
    rt: RuntimeHandle,
    component_id: ComponentId,
    component_fn: fn(Scope) -> Element,
    initialized: bool,
}

impl ReactivePreviewLive {
    fn new(component_fn: fn(Scope) -> Element) -> Self {
        let rt = RuntimeHandle::new();
        let component_id = rt.create_instance();
        Self {
            rt,
            component_id,
            component_fn,
            initialized: false,
        }
    }

    fn render_component(&mut self) -> Element {
        self.rt.reset_hook_cursor(self.component_id);
        self.rt.set_current_instance(Some(self.component_id));
        let scope = Scope::new(self.rt.clone(), self.component_id);
        let element = (self.component_fn)(scope);
        self.rt.set_current_instance(None);
        self.rt.clear_dirty();
        self.initialized = true;
        element
    }
}

impl LivePreview for ReactivePreviewLive {
    fn render(&mut self) -> Element {
        self.render_component()
    }

    fn needs_tick(&self) -> bool {
        // Reactive components with timelines need ticking
        true
    }

    fn tick(&mut self, _dt: f64) {
        // Timelines update themselves via Instant-based timing
        let _ = self.rt.with_instance(self.component_id, |_inst| {});
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        if key.code == KeyCode::Char('q') {
            return true;
        }
        self.rt.dispatch_input(key);
        true == false // never exit from dispatched input
    }
}

// ============================================================================
// Factory Function
// ============================================================================

pub fn create_live_preview(name: &str) -> Option<StdBox<dyn LivePreview>> {
    match name {
        // === Static (15) ===
        "banner" => Some(StdBox::new(StaticLive {
            build_fn: super::banner::build_ui,
        })),
        "barchart" => Some(StdBox::new(StaticLive {
            build_fn: super::barchart::build_ui,
        })),
        "borders" => Some(StdBox::new(StaticLive {
            build_fn: super::borders::build_ui,
        })),
        "breadcrumbs" => Some(StdBox::new(StaticLive {
            build_fn: super::breadcrumbs::build_ui,
        })),
        "diff" => Some(StdBox::new(StaticLive {
            build_fn: super::diff::build_ui,
        })),
        "gradient" => Some(StdBox::new(StaticLive {
            build_fn: super::gradient::build_ui,
        })),
        "hello" => Some(StdBox::new(StaticLive {
            build_fn: super::hello::build_ui,
        })),
        "keyhints" => Some(StdBox::new(StaticLive {
            build_fn: super::keyhints::build_ui,
        })),
        "markdown" => Some(StdBox::new(StaticLive {
            build_fn: super::markdown::build_ui,
        })),
        "modal" => Some(StdBox::new(StaticLive {
            build_fn: super::modal::build_ui,
        })),
        "statusbar" => Some(StdBox::new(StaticLive {
            build_fn: super::statusbar::build_ui,
        })),
        "syntax" => Some(StdBox::new(StaticLive {
            build_fn: super::syntax::build_ui,
        })),
        "table" => Some(StdBox::new(StaticLive {
            build_fn: super::table::build_ui,
        })),
        "tree" => Some(StdBox::new(StaticLive {
            build_fn: super::tree::build_ui,
        })),

        // === Timer-animated (9) ===
        "animation" => Some(StdBox::new(TimerLive {
            timer: AnimationTimer::new(),
            render_fn: super::animation::build_ui_with_timer,
        })),
        "demo_inline" => Some(StdBox::new(TimerLive {
            timer: AnimationTimer::new(),
            render_fn: super::demo_inline::build_ui_with_timer,
        })),
        "logbox" => Some(StdBox::new(TimerLive {
            timer: AnimationTimer::new(),
            render_fn: super::logbox::build_ui_with_timer,
        })),
        "logbox_command" => Some(StdBox::new(TimerLive {
            timer: AnimationTimer::new(),
            render_fn: super::logbox_command::build_ui_with_timer,
        })),
        "task_runner" => Some(StdBox::new(TimerLive {
            timer: AnimationTimer::new(),
            render_fn: super::task_runner::build_ui_with_timer,
        })),
        "spinner_demo" => Some(StdBox::new(TimerLive {
            timer: AnimationTimer::new(),
            render_fn: super::spinner_demo::build_ui_with_timer,
        })),
        "timer" => Some(StdBox::new(TimerLive {
            timer: AnimationTimer::new(),
            render_fn: super::timer::build_ui_with_timer,
        })),
        "preview" => Some(StdBox::new(TimerLive {
            timer: AnimationTimer::new(),
            render_fn: super::preview::build_ui_with_timer,
        })),
        "timeline_demo" => Some(StdBox::new(TimerLive {
            timer: AnimationTimer::new(),
            render_fn: super::timeline_demo::build_ui_with_timer,
        })),

        // === Interactive (4) ===
        "interactive" => Some(StdBox::new(InteractiveLive {
            state: super::interactive::AppState::new(),
        })),
        "focus_demo" => Some(StdBox::new(FocusDemoLive {
            state: super::focus_demo::AppState::new(),
        })),
        "form_demo" => Some(StdBox::new(FormDemoLive {
            state: super::form_demo::FormState::new(),
        })),
        "select_demo" => Some(StdBox::new(SelectDemoLive {
            state: super::select_demo::AppState::new(),
        })),

        // === Component-state (5) ===
        "menu" => Some(StdBox::new(MenuLive {
            items: super::menu::default_items(),
            state: SelectState::new(super::menu::default_items().len()),
        })),
        "tabs" => Some(StdBox::new(TabsLive {
            state: TabsState::new(super::tabs::TABS.len()),
        })),
        "multiselect" => Some(StdBox::new(MultiSelectLive {
            state: MultiSelectState::new(super::multiselect::ITEMS.len()),
        })),
        "autocomplete" => Some(StdBox::new(AutocompleteLive {
            state: AutocompleteState::with_count(super::autocomplete::SUGGESTIONS.len().min(6)),
        })),
        "polish_demo" => Some(StdBox::new(PolishDemoLive {
            confirm: ConfirmProps::new("Save changes?"),
        })),

        // === Sparkline (1) ===
        "sparkline" => Some(StdBox::new(SparklineLive {
            cpu_data: vec![
                30.0, 35.0, 40.0, 38.0, 42.0, 45.0, 50.0, 48.0, 52.0, 55.0, 53.0, 50.0, 47.0, 44.0,
                48.0, 52.0, 56.0, 60.0, 58.0, 55.0,
            ],
            mem_data: vec![
                45.0, 46.0, 47.0, 48.0, 49.0, 50.0, 51.0, 52.0, 53.0, 54.0, 55.0, 56.0, 57.0, 58.0,
                59.0, 60.0, 61.0, 62.0, 63.0, 64.0,
            ],
            net_data: vec![
                10.0, 15.0, 8.0, 20.0, 12.0, 25.0, 18.0, 30.0, 22.0, 35.0, 28.0, 40.0, 32.0, 45.0,
                38.0, 50.0, 42.0, 55.0, 48.0, 60.0,
            ],
            audio_data: vec![
                80.0, 45.0, 60.0, 30.0, 90.0, 50.0, 70.0, 40.0, 85.0, 55.0, 65.0, 35.0, 75.0, 48.0,
                88.0, 42.0, 72.0, 58.0, 82.0, 38.0, 68.0, 52.0, 78.0, 46.0,
            ],
            seed: 42,
            elapsed: 0.0,
        })),

        // === Stateful animated (4) ===
        "dashboard" => Some(StdBox::new(DashboardLive {
            state: super::dashboard::DashboardState::new(),
            params: super::dashboard::FieldParams::default(),
            last_render_ms: 0.0,
        })),
        "plasma" => Some(StdBox::new(PlasmaLive {
            params: super::plasma::Params::new(12345),
            lava: super::plasma::LavaLamp::new(8, 12345),
            time: 0.0,
        })),
        "showcase" => Some(StdBox::new(ShowcaseLive {
            state: super::showcase::ShowcaseState::new(),
        })),
        "cube3d_braille" => Some(StdBox::new(Cube3dLive {
            state: super::cube3d_braille::AppState::new(),
        })),

        // === Async (1) ===
        "async_app" => {
            let (event_tx, event_rx) = mpsc::channel();
            Some(StdBox::new(AsyncAppLive {
                state: super::async_app::AppState::new(),
                receiver: event_rx,
                event_tx,
            }))
        }

        // === Reactive (6) ===
        "reactive_counter" => Some(StdBox::new(ReactivePreviewLive::new(
            super::reactive_counter::counter,
        ))),
        "reactive_list" => Some(StdBox::new(ReactivePreviewLive::new(
            super::reactive_list::list_app,
        ))),
        "quickstart_interactive" => Some(StdBox::new(ReactivePreviewLive::new(
            super::quickstart_interactive::task_selector,
        ))),
        "reactive_timeline" => Some(StdBox::new(ReactivePreviewLive::new(
            super::reactive_timeline::animated_dashboard,
        ))),
        "stagger_demo" => Some(StdBox::new(ReactivePreviewLive::new(
            super::stagger_demo::component,
        ))),
        "timeline_debug" => Some(StdBox::new(ReactivePreviewLive::new(
            super::timeline_debug::debug_dashboard,
        ))),

        _ => None,
    }
}
