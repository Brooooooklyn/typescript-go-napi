//// [tests/cases/conformance/esDecorators/classExpression/classSuper/esDecorators-classExpression-classSuper.6.ts] ////

//// [esDecorators-classExpression-classSuper.6.ts]
declare var dec: any;

declare class Base {
    static method(...args: any[]): number;
    method(...args: any[]): number;
}

// none of the following should result in caching `super`
(@dec
class C extends Base {
    static m() { super.method(); }
    static get x() { return super.method(); }
    static set x(v: number) { super.method(); }

    constructor() {
        super();
        super.method();
    }

    a = super.method();
    m() { super.method(); }
    get x() { return super.method(); }
    set x(v: number) { super.method(); }
});


//// [esDecorators-classExpression-classSuper.6.js]
// none of the following should result in caching `super`
(
@dec
class C extends Base {
    static m() { super.method(); }
    static get x() { return super.method(); }
    static set x(v) { super.method(); }
    constructor() {
        super();
        super.method();
    }
    a = super.method();
    m() { super.method(); }
    get x() { return super.method(); }
    set x(v) { super.method(); }
});
