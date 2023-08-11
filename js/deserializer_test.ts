import { assertThrows, describe, it } from "./_test_deps.ts";
import { checkBuffer } from "./deserializer.ts";

describe("checkBuffer", () => {
  it("must reject empty buffer", () => {
    assertThrows(() => {
      checkBuffer(new Uint8Array());
    }, "Invalid length");
  });

  it("must reject FALSE that is not RLE-encoded", () => {
    Deno.test("rejects FALSE that is not RLE-encoded", () => {
      assertThrows(() => {
        checkBuffer(new Uint8Array([0]));
      }, "Invalid length");
    });
  });

  it("must accept FALSE that is RLE-encoded", () => {
    checkBuffer(new Uint8Array([0, 1]));
  });

  it("must disallow FALSE that has extra bytes", () => {
    assertThrows(() => {
      checkBuffer(new Uint8Array([0, 1, 25]));
    }, "Invalid length");
  });

  it("must disallow invalid types", () => {
    for (let i = 7; i <= 255; i++) {
      assertThrows(() => {
        checkBuffer(new Uint8Array([i]));
      }, "Invalid length");
    }
  });

  it("must accept valid types", () => {
    checkBuffer(new Uint8Array([1]));
    checkBuffer(new Uint8Array([2]));
    checkBuffer(new Uint8Array([3]));
    checkBuffer(new Uint8Array([4]));
    checkBuffer(new Uint8Array([5]));
    checkBuffer(new Uint8Array([6]));
  });
});
