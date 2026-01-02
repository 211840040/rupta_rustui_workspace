abstract class Element implements BuildContext {
  Element? _parent;
  Widget? _widget;
  Widget get widget => _widget!;
  bool _dirty = true;
  bool get dirty => _dirty;

  Element(Widget widget) : _widget = widget {
    print('Element');
  }
  void rebuild({bool force = false}) {
    print("Element::rebuild");
    if (dirty || force) {
      performRebuild();
    }
  }

  void performRebuild() {
    print("Element::perform_rebuild");
    _dirty = false;
  }

  void mount(Element? parent) {
    print("Element::mount");
    _parent = parent;
  }

  void markNeedsBuild() {
    print("Element::mark_needs_build");
    _dirty = true;
    rebuild(force: false);
  }
}

abstract class ComponentElement extends Element {
  Element? _child;
  ComponentElement(super.widget) {
    print('ComponentElement');
  }

  void _firstBuild() {
    print("ComponentElement::first_build");
    rebuild(force: false);
  }

  @override
  void performRebuild() {
    print("ComponentElement::perform_rebuild");
    build();
    super.performRebuild();
  }

  Widget build();

  @override
  void mount(Element? parent) {
    print("ComponentElement::mount");
    super.mount(parent);
    _firstBuild();
  }
}

class StatefulElement extends ComponentElement {
  State<StatefulWidget>? _state;
  State<StatefulWidget> get state => _state!;

  StatefulElement(StatefulWidget widget)
    : _state = widget.createState(),
      super(widget) {
    print('StatefulElement');
  }

  @override
  Widget build() => state.build(this);

  @override
  void _firstBuild() {
    print("StatefulElement::first_build");
    state.initState();
    super._firstBuild();
  }

  @override
  void performRebuild() {
    print("StatefulElement::perform_rebuild");
    super.performRebuild();
  }
}

abstract class State<T extends StatefulWidget> {
  T? _widget;
  T get widget => _widget!;

  StatefulElement? _element;

  void initState() {
    print("State::init_state");
  }

  Widget build(BuildContext context);

  void setState(void Function() fn) {
    print("State::set_state");
    fn();
    _element!.markNeedsBuild();
  }
}

abstract class BuildContext {
  Widget get widget;

  void mount(Element? parent) {
    print('BuildContext::mount');
  }
}

abstract class Widget {
  Element createElement();
}

abstract class StatefulWidget extends Widget {
  @override
  StatefulElement createElement() => StatefulElement(this);

  State createState();
}

class GalleryPage extends StatefulWidget {
  @override
  State<StatefulWidget> createState() {
    print('GalleryPage::create_state');
    return _GalleryPageState();
  }

  void onCreate() {
    print('GalleryPage::on_create');
  }
}

class _GalleryPageState extends State<GalleryPage> {
  @override
  void initState() {
    super.initState();
    print('GalleryPageState::init_state');
  }

  @override
  Widget build(BuildContext context) {
    print('GalleryPageState::build');
    (context.widget as GalleryPage).onCreate();
    return MyWidget();
  }
}

class MyWidget extends Widget {
  @override
  MyElement createElement() => MyElement(this);
}

class MyElement extends Element {
  MyElement(super.widget);
}

void main(List<String> arguments) {
  GalleryPage().createElement().mount(null);
}
