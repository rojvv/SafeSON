import { assertThrows, describe, it } from "./_test_deps.ts";
import { ARRAY, FALSE, NULL, NUMBER, OBJECT, STRING, TRUE } from "./const.ts";
import { checkBuffer } from "./deserializer.ts";
import { Deserializer } from "./mod.ts";

describe("checkBuffer", () => {
  it("must reject empty buffer", () => {
    assertThrows(() => {
      checkBuffer(new Uint8Array());
    }, "Invalid length");
  });

  it("must reject FALSE that is not RLE-encoded", () => {
    Deno.test("rejects FALSE that is not RLE-encoded", () => {
      assertThrows(() => {
        checkBuffer(new Uint8Array([FALSE]));
      }, "Invalid length");
    });
  });

  it("must accept FALSE that is RLE-encoded", () => {
    checkBuffer(new Uint8Array([FALSE, 1]));
  });

  it("must disallow FALSE that has extra bytes", () => {
    assertThrows(() => {
      checkBuffer(new Uint8Array([FALSE, 1, 25]));
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
    checkBuffer(new Uint8Array([TRUE]));
    checkBuffer(new Uint8Array([NULL]));
    checkBuffer(new Uint8Array([NUMBER]));
    checkBuffer(new Uint8Array([STRING]));
    checkBuffer(new Uint8Array([ARRAY]));
    checkBuffer(new Uint8Array([OBJECT]));
  });
});

describe("Deserializer.deserialize", () => {
  it("must throw on data shortage", () => {
    assertThrows(() => {
      Deserializer.deserialize(new Uint8Array([STRING]));
    }, "No more data remaining");

    assertThrows(() => {
      Deserializer.deserialize(new Uint8Array([STRING, 10]));
    }, "No more data remaining");
  });

  it("must throw on extra bytes", () => {
    assertThrows(() => {
      Deserializer.deserialize(new Uint8Array([TRUE, 1]));
    });
  });
});
