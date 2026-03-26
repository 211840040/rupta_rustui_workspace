use std::cell::RefCell;
use std::time::Duration;

classes::classes! {
    pub abstract class BindingBase {
        struct {
            lock_count: u32 = 0_u32,
        }

        pub fn new() -> Self {
            let self = Self { .. };
            self.init_instances();
            self
        }
        pub fn init_instances(&self) {
        }

        pub fn check_instance(instance: Option<CRc<Self>>) -> CRc<Self> {
            instance.expect("Binding mixin instance is None but bindings are already initialized")
        }
    }

    // ========== SchedulerBinding ==========

    pub const TIME_DILATION: u32 = 1;

    static SCHEDULING_BINDING_INSTANCE: SyncRefCell<Option<CRc<SchedulerBinding>>> =
        SyncRefCell::new(core::cell::RefCell::new(None));

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u32)]
    pub enum SchedulerPhase {
        Idle,
        TransientCallbacks,
        MidFrameMicrotasks,
        PersistentCallbacks,
        PostFrameCallbacks,
    }

    impl SchedulerPhase {
        pub fn index(self) -> u32 {
            self as u32
        }
    }

    #[with(BindingBase)]
    pub mixin SchedulerBinding on BindingBase {

        struct {
            timings_callbacks_count: usize = 0_usize,
            has_requested_an_event_loop_callback: bool = false,
            next_frame_callback_id: usize = 0_usize,
            has_scheduled_frame: bool = false,
            frame_callback_registered: bool = false,
            scheduler_phase: SchedulerPhase = SchedulerPhase::Idle,
            frames_enabled: bool = true,
            warm_up_frame: bool = false,
            first_raw_time_stamp_in_epoch: Option<Duration> = None,
            epoch_start: Duration = Duration::ZERO,
            last_raw_time_stamp: Duration = Duration::ZERO,
            current_frame_time_stamp: Option<Duration> = None,
            reschedule_after_warmup_frame: bool = false,
            debug_frame_number: usize = 0_usize,
            num_performance_mode_requests: usize = 0_usize,
        }
        pub override fn BindingBase::init_instances(&self) {
            super.init_instances();
            *SCHEDULING_BINDING_INSTANCE.borrow_mut() = Some(self.to_mixin());
        }


        #[inline]
        pub fn instance() -> CRc<SchedulerBinding> {
            BindingBase::check_instance(
                SCHEDULING_BINDING_INSTANCE
                    .borrow()
                    .as_ref()
                    .map(|binding| binding.mixin_to_impl())
            ).cast_mixin()
        }
    }


    // ========== GestureBinding ==========

    static GESTURE_BINDING_INSTANCE: SyncRefCell<Option<CRc<GestureBinding>>> =
        SyncRefCell::new(core::cell::RefCell::new(None));

    #[with(BindingBase/SchedulerBinding)]
    pub mixin GestureBinding on BindingBase {

        struct {
            resampling_enabled: bool = false,
        }

        pub override fn BindingBase::init_instances(&self) {
            super.init_instances();
            *GESTURE_BINDING_INSTANCE.borrow_mut() = Some(self.to_mixin());
        }


        pub fn instance() -> CRc<GestureBinding> {
            BindingBase::check_instance(
                GESTURE_BINDING_INSTANCE
                    .borrow()
                    .as_ref()
                    .map(|binding| binding.mixin_to_impl())
            ).cast_mixin()
        }

        pub fn get_instance() -> Option<CRc<GestureBinding>> {
            GESTURE_BINDING_INSTANCE.borrow().clone()
        }
    }

    // ========== ServicesBinding ==========

    static SERVICE_BINDING_INSTANCE: SyncRefCell<Option<CRc<ServicesBinding>>> =
        SyncRefCell::new(core::cell::RefCell::new(None));

    #[with(BindingBase/SchedulerBinding/GestureBinding)]
    pub mixin ServicesBinding on BindingBase, SchedulerBinding {



        pub override fn BindingBase::init_instances(&self) {
            super.init_instances();
            *SERVICE_BINDING_INSTANCE.borrow_mut() = Some(self.to_mixin());
        }


        pub fn instance() -> CRc<ServicesBinding> {
            BindingBase::check_instance(
                SERVICE_BINDING_INSTANCE
                    .borrow()
                    .as_ref()
                    .map(|binding| binding.mixin_to_impl())
            ).cast_mixin()
        }


    }

    #[with(Object)]
    pub mixin SystemContextMenuClient {
        pub fn handle_system_hide(&self);
    }

    // ========== PaintingBinding ==========

    static PAITING_BINDING_INSTANCE: SyncRefCell<Option<CRc<PaintingBinding>>> =
        SyncRefCell::new(core::cell::RefCell::new(None));


    #[with(BindingBase/SchedulerBinding/GestureBinding/ServicesBinding)]
    pub mixin PaintingBinding on BindingBase, ServicesBinding {
        pub override fn BindingBase::init_instances(&self) {
            super.init_instances();
            *PAITING_BINDING_INSTANCE.borrow_mut() = Some(self.to_mixin());
        }

        fn get_instance() -> Option<CRc<PaintingBinding>> {
            PAITING_BINDING_INSTANCE.borrow().clone()
        }

        pub fn instance() -> CRc<PaintingBinding> {
            BindingBase::check_instance(
                PAITING_BINDING_INSTANCE
                    .borrow()
                    .as_ref()
                    .map(|binding| binding.mixin_to_impl())
            ).cast_mixin()
        }
    }

    // ========== SemanticsBinding ==========

    static SEMANTICS_BINDING_INSTANCE: SyncRefCell<Option<CRc<SemanticsBinding>>> =
        SyncRefCell::new(RefCell::new(None));

    #[with(BindingBase/SchedulerBinding/GestureBinding/ServicesBinding/PaintingBinding)]
    pub mixin SemanticsBinding on BindingBase {

        struct {
            outstanding_handles: u32 = 0_u32,
        }

        pub override fn BindingBase::init_instances(&self) {
            super.init_instances();
            *SEMANTICS_BINDING_INSTANCE.borrow_mut() = Some(self.to_mixin());
        }


        pub fn instance() -> CRc<SemanticsBinding> {
            BindingBase::check_instance(
                SEMANTICS_BINDING_INSTANCE
                    .borrow()
                    .as_ref()
                    .map(|binding| binding.mixin_to_impl())
            ).cast_mixin()
        }

        fn get_instance() -> Option<CRc<SemanticsBinding>> {
            SEMANTICS_BINDING_INSTANCE.borrow().clone()
        }
    }

    // ========== RendererBinding ==========

    static RENDERING_BINDING_INSTANCE: SyncRefCell<Option<CRc<RendererBinding>>> =
        SyncRefCell::new(core::cell::RefCell::new(None));

    #[with(BindingBase/SchedulerBinding/GestureBinding/ServicesBinding/PaintingBinding/SemanticsBinding)]
    pub mixin RendererBinding
    on
        BindingBase,
        ServicesBinding,
        SchedulerBinding,
        GestureBinding,
        SemanticsBinding
    {
        struct {
            first_frame_deferred_count: i32 = 0,
            first_frame_sent: bool = false,
        }

        pub override fn BindingBase::init_instances(&self) {
            super.init_instances();
            *RENDERING_BINDING_INSTANCE.borrow_mut() = Some(self.to_mixin());
        }



        pub fn instance() -> CRc<RendererBinding> {
            BindingBase::check_instance(
                RENDERING_BINDING_INSTANCE
                    .borrow()
                    .as_ref()
                    .map(|binding| binding.mixin_to_impl())
            ).cast_mixin()
        }

        pub fn get_instance() -> Option<CRc<RendererBinding>> {
            RENDERING_BINDING_INSTANCE.borrow().clone()
        }
    }

    pub class RenderingFlutterBinding extends BindingBase
        with
            SchedulerBinding,
            GestureBinding,
            ServicesBinding,
            PaintingBinding,
            SemanticsBinding,
            RendererBinding
    {
        fn new() -> Self {
            Self { super: Super::new(), .. }
        }

        pub fn ensure_initialized() -> CRc<RendererBinding> {
            if RendererBinding::get_instance().is_none() {
                let _binding = CRc::<RenderingFlutterBinding>::new();
            }
            RendererBinding::instance()
        }
    }


    static WIDGETS_BINDING_INSTANCE: SyncRefCell<Option<CRc<WidgetsBinding>>> =
        SyncRefCell::new(core::cell::RefCell::new(None));

    #[with(BindingBase/SchedulerBinding/GestureBinding/ServicesBinding/PaintingBinding/SemanticsBinding/RendererBinding)]
    pub mixin WidgetsBinding
    on
        BindingBase,
        ServicesBinding,
        SchedulerBinding,
        GestureBinding,
        RendererBinding,
        SemanticsBinding
    {
        struct {
            need_to_report_first_frame: bool = true,
            ready_to_produce_frames: bool = false,
            debug_building_dirty_elements: bool = false,
            debug_show_widget_inspector_override: bool = false,
            debug_exclude_root_widget_inspector: bool = false,
        }

        pub override fn BindingBase::init_instances(&self) {
            super.init_instances();
            *WIDGETS_BINDING_INSTANCE.borrow_mut() = Some(self.to_mixin());
        }



        fn get_instance() -> Option<CRc<WidgetsBinding>> {
            WIDGETS_BINDING_INSTANCE.borrow().clone()
        }

        pub fn instance() -> CRc<WidgetsBinding> {
            BindingBase::check_instance(
                WIDGETS_BINDING_INSTANCE
                    .borrow()
                    .as_ref()
                    .map(|binding| binding.mixin_to_impl())
            ).cast_mixin()
        }
    }

    pub fn run_app() {
        let _ = WidgetsFlutterBinding::ensure_initialized();
    }



    class WidgetsFlutterBinding extends BindingBase
    with
        SchedulerBinding,
        GestureBinding,
        ServicesBinding,
        PaintingBinding,
        SemanticsBinding,
        RendererBinding,
        WidgetsBinding
    {
        fn new() -> Self {
            Self { super: Super::new() }
        }

        pub fn ensure_initialized() -> CRc<WidgetsBinding> {
            if WidgetsBinding::get_instance().is_none() {
                let _binding = CRc::<WidgetsFlutterBinding>::new();
            }
            WidgetsBinding::instance()
        }
    }
}

use core::ops::{Deref, DerefMut};

pub type SyncOnceCell<T> = SyncWrapper<core::cell::OnceCell<T>>;
pub type SyncRefCell<T> = SyncWrapper<core::cell::RefCell<T>>;
pub type SyncCell<T> = SyncWrapper<core::cell::Cell<T>>;
pub type SyncLazyCell<T> = SyncWrapper<core::cell::LazyCell<T>>;

#[derive(Default)]
struct ConcurrentAccessGuard {
    thread_id: std::sync::OnceLock<std::thread::ThreadId>,
}

impl ConcurrentAccessGuard {
    const fn new() -> Self {
        Self {
            thread_id: std::sync::OnceLock::new(),
        }
    }

    fn guard(&self) {
        #[inline(never)]
        #[cold]
        fn concurrent_access() -> ! {
            panic!("Concurrent access to a `SyncOnceCell`")
        }

        let thread_id = std::thread::current().id();
        if thread_id != *self.thread_id.get_or_init(|| thread_id) {
            concurrent_access();
        }
    }
}

#[derive(Default)]
pub struct SyncWrapper<T> {
    inner: T,
    guard: ConcurrentAccessGuard,
}

unsafe impl<T> Sync for SyncWrapper<T> {}

impl<T> Deref for SyncWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.guard.guard();
        &self.inner
    }
}

impl<T> DerefMut for SyncWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.guard();
        &mut self.inner
    }
}

impl<T> SyncWrapper<T> {
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
            guard: ConcurrentAccessGuard::new(),
        }
    }
    pub fn into_inner(self) -> T {
        self.guard.guard();
        self.inner
    }
}
