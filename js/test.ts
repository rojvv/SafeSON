import { assertEquals } from "./_test_deps.ts";
import { Serializer } from "./serializer.ts";
import { Deserializer } from "./deserializer.ts";

// deno-lint-ignore no-explicit-any
function v(v: any) {
  const expected = JSON.stringify(v);
  // deno-fmt-ignore
  const actual = JSON.stringify(Deserializer.desrialize(Serializer.serialize(v)));
  assertEquals(actual, expected);
}

Deno.test("boolean", () => {
  v(false);
  v(true);
});

Deno.test("null", () => {
  v(null);
});

Deno.test("number", () => {
  v(-100);
  v(0);
  v(0xFFFF);
  v(10e15);
});

Deno.test("string", () => {
  v("Hello, world!");
  v("Hello, world!".repeat(40));
});

Deno.test("array", () => {
  v([
    false,
    true,
    null,
    -100,
    0,
    0xFFFF,
    10e15,
    "Hello, world!",
    "Hello, world!".repeat(40),
    {
      a: false,
      b: true,
      c: null,
      d: -100,
      e: 0,
      f: 0xFFFF,
      g: 10e15,
      h: "Hello, world!",
      i: "Hello, world!".repeat(40),
      j: [false, true, null],
    },
  ]);
});

Deno.test("object", () => {
  v({
    a: false,
    b: true,
    c: null,
    d: -100,
    e: 0,
    f: 0xFFFF,
    g: 10e15,
    h: "Hello, world!",
    i: "Hello, world!".repeat(40),
    j: [false, true, null],
    k: [
      false,
      true,
      null,
      -100,
      0,
      0xFFFF,
      10e15,
      "Hello, world!",
      "Hello, world!".repeat(40),
      {
        a: false,
        b: true,
        c: null,
        d: -100,
        e: 0,
        f: 0xFFFF,
        g: 10e15,
        h: "Hello, world!",
        i: "Hello, world!".repeat(40),
        j: [false, true, null],
      },
    ],
  });
});
