type Result<T, E> = { status: "ok", data: T} | { status: "error", error: E };

export async function unwrap<T, E>(p: Promise<Result<T, E>>): Promise<T> {
    const r = await p;
    if (r.status === "error") throw r.error;
    return r.data;
}