import { backend } from "../../../declarations/backend";

export interface ChatMessage {
  role: string;
  content: string;
}

export interface SupabaseResponse {
  data: [] | [string];
  error: [] | [string];
}

export interface QueryParseResult {
  table: string;
  query: string;
  error: [] | [string];
}

export async function greet(name: string): Promise<string> {
  try {
    return await backend.greet(name);
  } catch (error) {
    console.error("Failed to greet:", error);
    throw error;
  }
}

export async function getCount(): Promise<bigint> {
  try {
    return await backend.get_count();
  } catch (error) {
    console.error("Failed to get count:", error);
    throw error;
  }
}

export async function increment(): Promise<bigint> {
  try {
    return await backend.increment();
  } catch (error) {
    console.error("Failed to increment:", error);
    throw error;
  }
}

export async function setCount(value: bigint): Promise<bigint> {
  try {
    return await backend.set_count(value);
  } catch (error) {
    console.error("Failed to set count:", error);
    throw error;
  }
}

export async function chat(messages: ChatMessage[]): Promise<string> {
  try {
    return await backend.chat(messages);
  } catch (error) {
    console.error("Failed to chat:", error);
    throw error;
  }
}

export async function querySupabaseWithNaturalLanguage(
  query: string,
): Promise<any[]> {
  try {
    const result = await backend.query_supabase_with_natural_language(query);

    if ("Ok" in result) {
      const response = result.Ok;
      // Handle optional array types properly
      if (response.error && response.error.length > 0 && response.error[0]) {
        throw new Error(response.error[0]);
      }

      if (response.data && response.data.length > 0 && response.data[0]) {
        const data = JSON.parse(response.data[0]);
        return Array.isArray(data) ? data : [data];
      }

      return [];
    } else {
      throw new Error(result.Err);
    }
  } catch (error) {
    console.error("Failed to query Supabase with natural language:", error);
    throw error;
  }
}

export async function parseNaturalLanguageQuery(
  query: string,
): Promise<QueryParseResult> {
  try {
    const result = await backend.debug_parse_query(query);

    if ("Ok" in result) {
      return result.Ok;
    } else {
      throw new Error(result.Err);
    }
  } catch (error) {
    console.error("Failed to parse natural language query:", error);
    throw error;
  }
}
