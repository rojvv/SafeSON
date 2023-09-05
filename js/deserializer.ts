import { ARRAY, FALSE, NULL, NUMBER, OBJECT, STRING, TRUE } from "./const.ts";
import { rleDecode } from "./rle.ts";

export class DeserializationError extends Error {
  constructor(message: string) {
    super(message);
  }
}

export function checkBuffer(b: Uint8Array) {
  if (
    b.length <= 0 ||
    ((b[0] == TRUE || b[0] == NULL) && b.length != 1) ||
    (b[0] == FALSE && (b.length != 2 || b[1] != 1))
  ) {
    throw new DeserializationError("Invalid length");
  } else if (
    b[0] != NULL && b[0] != TRUE && b[0] != FALSE && b[0] != NUMBER &&
    b[0] != STRING && b[0] != ARRAY &&
    b[0] != OBJECT
  ) {
    throw new DeserializationError("Invalid type");
  }
}

export class Deserializer {
  private _buffer = new Uint8Array();
  private offset = 0;

  private read(count: number) {
    const b = this._buffer.slice(this.offset, this.offset + count);
    if (b.length < count) {
      throw new DeserializationError("No more data remaining");
    }
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
      const length = this.readNumber();
      if (length < 0 || length % 1 != 0) {
        throw new DeserializationError("Invalid length");
      } else {
        return length;
      }
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
        throw new DeserializationError(`Invalid type: ${type}`);
    }
  }

  static deserialize(b: Uint8Array) {
    checkBuffer(b);
    const deserializer = new Deserializer();
    deserializer._buffer = rleDecode(b);
    const v = deserializer.readValue();
    if (deserializer._buffer.length > deserializer.offset) {
      throw new DeserializationError("Extra bytes");
    }
    // deno-lint-ignore no-explicit-any
    return v as any;
  }
}
