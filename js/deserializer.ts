import { ARRAY, FALSE, NULL, NUMBER, OBJECT, STRING, TRUE } from "./const.ts";

export class DeserializationError extends Error {
  constructor(message: string, public readonly byteIndex: number) {
    super(message);
  }
}

export class Deserializer {
  private _buffer = new Uint8Array();
  private offset = 0;

  private read(count: number) {
    const b = this._buffer.slice(this.offset, this.offset + count);
    this.offset += b.length;
    return b;
  }

  private readNumber() {
    const b = this.read(8);
    const v = new DataView(b.buffer);
    return v.getFloat64(0, true);
  }

  private readLength() {
    const b = this.read(1)[0];
    if (b <= 254) {
      return b;
    } else {
      return this.readNumber();
    }
  }

  private readString() {
    const length = this.readLength();
    return new TextDecoder().decode(this.read(length));
  }

  private readArray() {
    const length = this.readLength();
    // deno-lint-ignore no-explicit-any
    const array = new Array<any>();
    for (let i = 0; i < length; i++) {
      array.push(this.readValue());
    }
    return array;
  }

  private readObject() {
    const length = this.readLength();
    // deno-lint-ignore no-explicit-any
    const object = {} as Record<string, any>;
    for (let i = 0; i < length; i++) {
      const key = this.readString();
      object[key] = this.readValue();
    }
    return object;
  }

  private readValue() {
    const type = this.read(1)[0];
    switch (type) {
      case FALSE:
        return false;
      case TRUE:
        return true;
      case NULL:
        return null;
      case NUMBER:
        return this.readNumber();
      case STRING:
        return this.readString();
      case ARRAY:
        return this.readArray();
      case OBJECT:
        return this.readObject();
      default:
        throw new DeserializationError(
          `Invalid type: ${type}`,
          this.offset - 1,
        );
    }
  }
}
