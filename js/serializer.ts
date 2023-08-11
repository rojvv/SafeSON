import { ARRAY, FALSE, NULL, NUMBER, OBJECT, STRING, TRUE } from "./const.ts";

export class Serializer {
  private _buffer = new Array<number>();

  private write(b: Iterable<number>) {
    for (const i of b) {
      this._buffer.push(i);
    }
  }

  private writeBoolean(b: boolean) {
    if (b) {
      this.write([TRUE]);
    } else {
      this.write([FALSE]);
    }
  }

  private writeNull() {
    this.write([NULL]);
  }

  private writeNumberInner(number: number) {
    const b = new Uint8Array(8);
    const v = new DataView(b.buffer);
    v.setFloat64(0, number, true);
    this.write(b);
  }

  private writeNumber(number: number) {
    this.write([NUMBER]);
    this.writeNumberInner(number);
  }

  private writeLength(length: number) {
    if (length <= 254) {
      this.write([length]);
    } else {
      this.write([255]);
      this.writeNumberInner(length);
    }
  }

  private writeStringInner(string: string) {
    const b = new TextEncoder().encode(string);
    this.writeLength(b.byteLength);
    this.write(b);
  }

  private writeString(string: string) {
    this.write([STRING]);
    this.writeStringInner(string);
  }

  // deno-lint-ignore no-explicit-any
  private writeArray(array: any[]) {
    this.write([ARRAY]);
    this.writeLength(array.length);
    for (const i of array) {
      this.writeValue(i);
    }
  }

  // deno-lint-ignore no-explicit-any
  private writeObject(object: Record<string, any>) {
    this.write([OBJECT]);
    this.writeLength(Object.keys(object).length);
    for (const [k, v] of Object.entries(object)) {
      this.writeStringInner(k);
      this.writeValue(v);
    }
  }

  // deno-lint-ignore no-explicit-any
  private writeValue(value: any) {
    if (typeof value === "boolean") {
      this.writeBoolean(value);
    } else if (value === null) {
      this.writeNull();
    } else if (typeof value === "number") {
      this.writeNumber(value);
    } else if (typeof value === "string") {
      this.writeString(value);
    } else if (Array.isArray(value)) {
      this.writeArray(value);
    } else {
      this.writeObject(value);
    }
  }

  // deno-lint-ignore no-explicit-any
  static serialize(value: any) {
    const serializer = new Serializer();
    serializer.writeValue(value);
    return serializer._buffer;
  }
}
