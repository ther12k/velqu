/**
 * @q/core — static authoring primitives. Pure data constructors: no side
 * effects, no I/O, no server runtime code. The compiler reads the literal
 * arguments of these calls WITHOUT executing handlers or service factories
 * (COMP-002); `handle`/`check`/factories never run at build time.
 */
export class Status {
    status;
    constructor(status) {
        this.status = status;
    }
    value(value) {
        return { __ok: true, status: this.status, value };
    }
    problem(problem, opts = {}) {
        return { __problem: true, problem, status: this.status, ...opts };
    }
}
/** `status(201).value(user)` / `status(404).problem("not-found")` */
export function status(code) {
    return new Status(code);
}
/** Route constructor. `def` is read statically by the compiler. */
export function route(def) {
    return { __route: true, ...def };
}
export function definePolicy(def) {
    return { __policy: true, ...def };
}
export function defineService(id, factory) {
    return { __service: true, id, factory };
}
export function defineModule(def) {
    return { __module: true, ...def };
}
export function defineApp(def) {
    return { __app: true, ...def };
}
