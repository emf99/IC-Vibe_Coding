import { describe, it, expect, beforeAll, afterAll, inject } from "vitest";
import { PocketIc } from "@dfinity/pic";
import { _SERVICE } from "../../src/declarations/backend/backend.did.d.ts";
import { Principal } from "@dfinity/principal";
import { ActorSubclass } from "@dfinity/agent";
import { readFileSync } from "fs";
import { resolve, dirname } from "path";
import { fileURLToPath } from "url";

describe("Backend Service", () => {
  let pic: PocketIc;
  let actor: ActorSubclass<_SERVICE>;
  let canisterId: Principal;

  beforeAll(async () => {
    // Initialize PocketIC with default configuration - let it manage the server
    pic = await PocketIc.create(inject("PIC_URL"));

    // Load the backend WASM file
    const wasmPath = resolve(
      dirname(fileURLToPath(import.meta.url)),
      "..",
      "..",
      "target",
      "wasm32-unknown-unknown",
      "release",
      "backend.wasm",
    );

    let wasmModule: Buffer;
    try {
      wasmModule = readFileSync(wasmPath);
    } catch (error) {
      throw new Error(
        `Failed to read WASM file at ${wasmPath}. Run 'dfx build backend' first.`,
      );
    }

    // Load the IDL factory
    const { idlFactory } = await import(
      "../../src/declarations/backend/backend.did.js"
    );

    // Create and install the backend canister
    const fixture = await pic.setupCanister<_SERVICE>({
      idlFactory,
      wasm: wasmModule,
    });

    actor = fixture.actor;
    canisterId = fixture.canisterId;
  });

  afterAll(async () => {
    if (pic) {
      await pic.tearDown();
    }
  });

  describe("greet", () => {
    it("should return a greeting message", async () => {
      const result = await actor.greet("World");
      expect(result).toContain("Hello");
      expect(result).toContain("World");
    });
  });

  describe("counter", () => {
    it("should get the current count", async () => {
      const count = await actor.get_count();
      expect(typeof count).toBe("bigint");
      expect(count).toBeGreaterThanOrEqual(0n);
    });

    it("should increment the counter", async () => {
      const initialCount = await actor.get_count();
      const newCount = await actor.increment();
      expect(newCount).toBe(initialCount + 1n);
    });

    it("should set the counter to a specific value", async () => {
      const targetValue = 42n;
      const result = await actor.set_count(targetValue);
      expect(result).toBe(targetValue);

      const currentCount = await actor.get_count();
      expect(currentCount).toBe(targetValue);
    });
  });

  describe("chat", () => {
    it("should process chat messages", async () => {
      const messages = [{ role: "user", content: "Hello, how are you?" }];

      const response = await actor.chat(messages);
      expect(typeof response).toBe("string");
      expect(response.length).toBeGreaterThan(0);
    });
  });

  describe("natural language queries", () => {
    it("should parse natural language queries", async () => {
      const query = "get all todos";
      const result = await actor.debug_parse_query(query);

      if ("Ok" in result) {
        expect(result.Ok).toHaveProperty("table");
        expect(result.Ok).toHaveProperty("query");
      } else {
        expect(result.Err).toBeDefined();
        expect(typeof result.Err).toBe("string");
      }
    });
  });
});
