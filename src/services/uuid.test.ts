import { afterEach, describe, expect, it, vi } from "vitest";
import { createUuid } from "./uuid";

const uuidPattern =
  /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/;

afterEach(() => {
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
});

describe("createUuid", () => {
  it("uses the native implementation when available", () => {
    const expected = "11111111-1111-4111-8111-111111111111";
    vi.spyOn(crypto, "randomUUID").mockReturnValue(expected);

    expect(createUuid()).toBe(expected);
  });

  it("creates a valid version-four UUID without randomUUID", () => {
    vi.stubGlobal("crypto", {
      getRandomValues(array: Uint8Array) {
        array.fill(0xab);
        return array;
      },
    });

    const uuid = createUuid();
    expect(uuid).toBe("abababab-abab-4bab-abab-abababababab");
    expect(uuid).toMatch(uuidPattern);
  });
});
