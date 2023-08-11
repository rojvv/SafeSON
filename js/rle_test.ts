import { assertEquals } from "./_test_deps.ts";
import { rleDecode, rleEncode } from "./rle.ts";

Deno.test("rleEncode", () => {
  // deno-fmt-ignore
  const actual = rleEncode(new TextEncoder().encode("\x00".repeat(1000) + "\x01"));
  // deno-fmt-ignore
  const expected = new Uint8Array([0, 255, 0, 255, 0, 255, 0, 235, 1]);
  assertEquals(actual, expected);
});

Deno.test("rleDecode", () => {
  // deno-fmt-ignore
  const actual = rleDecode(new Uint8Array([0, 255, 0, 255, 0, 200, 3, 2, 1, 0, 2]));
  // deno-fmt-ignore
  const expected = new Uint8Array([...new TextEncoder().encode("\x00".repeat(710)), 3, 2, 1, 0, 0]);
  assertEquals(actual, expected);
});
