# SafeSON Specification

- [Format](#format)
  - [Type ID](#type-id)
  - [Number](#number)
  - [Length](#length)
  - [String](#string)
  - [Array](#array)
  - [Entry](#entry)
  - [Object](#object)
  - [Value](#value)
- [Serialization](#serialization)
  - [1. Serializing the Values](#1-serializing-the-values)
    - [Booleans](#booleans)
    - [Nulls](#nulls)
    - [Numbers](#numbers)
    - [Strings](#strings)
    - [Arrays](#arrays)
    - [Objects](#objects)
  - [2. RLE Encoding](#2-rle-encoding)
- [Deserialization](#deserialization)
  - [1. Validating the Payload](#1-validating-the-payload)
  - [2. RLE Decoding](#2-rle-decoding)
  - [3. Notes](#3-notes)

## Format

### Type ID

A Type ID is one of these byte literals:

- `0x00` FALSE
- `0x01` TRUE
- `0x02` NULL
- `0x03` NUMBER
- `0x04` STRING
- `0x05` ARRAY
- `0x06` OBJECT

### Number

A Number is a 64-bit double-precision floating-point value in the little-endian
order.

### Length

A Length specifies the number of bytes in a [String](#string), [Values](#value)
in an array, or [Entries](#value) in an object.

If that number is less than or equal to 254, it is denoted with a single byte
holding that number.

Otherwise, it is the byte `0xFF` followed by a [Number](#number) which holds
that number.

### String

A String is a [Length](#length) followed by a UTF-8 encoded string of the
specified length.

### Array

An Array is a [Length](#length) followed by a sequence of [Values](#entry) of
the specified length.

### Entry

An Entry is a [String](#string) followed by a [Value](#value).

### Object

An Object is a [Length](#length) followed by a sequence of [Entries](#entry) of
the specified length.

### Value

A Value is one of these:

- The Type ID for FALSE.
- The Type ID for TRUE.
- The Type ID for NULL.
- The Type ID for NUMBER, followed by a [Number](#number).
- The Type ID for STRING, followed by a [String](#string).
- The Type ID for ARRAY, followed by an [Array](#array).
- The Type ID for OBJECT, followed by an [Object](#object).

## Serialization

### 1. Serializing the Values

Several data types can be serialized:

#### Booleans

- The boolean values “false” and “true” are serialized by swapping them with the
  Type IDs for FALSE, and TRUE, respectively.

#### Nulls

- Nulls or non-existent values are serialized by swapping them with the Type ID
  for NULL.

#### Numbers

- Numeric types must first be converted to 64-bit floats.
- Numeric types are serialized by putting their byte representation in the
  little-endian order after the Type ID for NUMBER.

#### Strings

- A string is serialized by the Type ID for STRING, followed by its length as
  [Length](#length), and its UTF-8 bytes.

#### Arrays

- Arrays (unkeyed sequences) are serializing by the Type ID for STRING, followed
  by the number of [Values](#value) it is holding as [Length](#length), and
  those [Values](#value).

#### Objects

- Objects (keyed sequences) are serialized by the Type ID for OBJECT, followed
  by the number of [Entries](#entry) it is holding as [Length](#length), and
  those [Entries](#entry).

### 2. RLE Encoding

The serialized payloads must undergo a run-length encoding process on bytes with
the value `0x00`. Here’s a pseudo code for that:

```python
run = 0

for byte in serialized:
    if byte == 0x00:
        if run == 0xFF:
            result.concat(0x00)
            result.concat(run)
            run = 1
        else:
            run += 1
    else:
        if run != 0:
            result.concat(0x00)
            result.concat(run)
            run = 0

        result.concat(byte)

if run != 0:
    result.concat(0x00)
    result.concat(run)
```

## Deserialization

### 1. Validating the Payload

Before the deserialization process begins, the length and first one or two bytes
of the payload must be validated:

1. Reject if it has no bytes.
2. Reject if it starts with the Type ID for FALSE and but its length is not 2,
   or its second byte is not `0x01`.
3. Reject if it starts with the Type ID for TRUE or the Type ID for NULL but its
   length is not 1.
4. Reject if it doesn’t start with a valid [Type ID](#type-id).

### 2. RLE Decoding

This step is reversing what was done in [RLE Encoding](#2-rle-encoding). Here’s
a pseudo code for that:

```python
run = false

for byte in serialized:
    if byte == 0x00:
        run = true
        continue

    if run:
        for i = 0; i < byte; i += 1:
            result.concat(0x00)

        run = false
    else:
        result.concat(byte)
```

### 3. Notes

- The deserialization process must fail if the payload had extra bytes.
- The deserialization process must fail if a decoded [Length](#length) was
  smaller than 0 (a negative value) or had numbers other than zero after the
  point.
