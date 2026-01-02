abstract class I {
  void i();
}

abstract class A {
  final int x = 0;
  void f() {
    print('A::f, x = $x');
  }
}

abstract class B extends A {
  final int y = 1;
  @override
  void f() {
    super.f();
    print('B::f, y = $y');
  }

  void g() {
    print('B::g, y = $y');
  }
}

mixin M on A implements I {
  final int z = 2;
  @override
  void f() {
    super.f();
    print('M::f, z = $z');
  }

  void h() {
    print('M::h, z = $z');
  }
}

class C1 extends A with M implements I {
  final w = 3;
  @override
  void f() {
    super.f();
    print('C1::f, w = $w');
  }

  @override
  void h() {
    super.h();
    print('C1::h, w = $w');
  }

  @override
  void i() {
    print('C1::i, w = $w');
  }

  void j() {
    print('C1::j, w = $w');
  }
}

class C2 extends B with M implements I {
  final v = 4;
  @override
  void f() {
    super.f();
    print('C2::f, v = $v');
  }

  @override
  void g() {
    super.g();
    print('C2::g, v = $v');
  }

  @override
  void h() {
    super.h();
    print('C2::h, v = $v');
  }

  @override
  void i() {
    print('C2::i, v = $v');
  }

  void j() {
    print('C2::j, v = $v');
  }
}

void main() {
  final c1 = C1();
  c1.f();
  c1.h();
  c1.i();
  c1.j();

  M c1m = c1;
  c1m.f();
  c1m.h();
  print("z = ${c1m.z}");

  final c2 = C2();
  c2.f();
  c2.g();
  c2.h();
  c2.i();
  c2.j();

  M c2m = c2;
  c2m.f();
  c2m.h();
  print("z = ${c2m.z}");
}
