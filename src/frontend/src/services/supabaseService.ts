import { backend } from "../../../declarations/backend";

export interface SupabaseResponse {
  data?: any;
  error?: string;
}

export async function fetchSupabaseData(
  table: string,
  query: string = "",
): Promise<SupabaseResponse> {
  try {
    const response = await backend.fetch_from_supabase(table, query);
    if ("Ok" in response) {
      const result = response.Ok;

      // Handle Candid optional types: [] | [T] where [] means None and [T] means Some(T)
      const data = result.data.length > 0 ? result.data[0] : null;
      const error = result.error.length > 0 ? result.error[0] : undefined;

      if (data) {
        try {
          const parsedData = JSON.parse(data);
          return { data: parsedData, error };
        } catch {
          // If parsing fails, return as string
          return { data, error };
        }
      }
      return { data: null, error };
    } else {
      return { error: response.Err };
    }
  } catch (error) {
    console.error("Failed to fetch from Supabase:", error);
    return { error: "Failed to fetch data" };
  }
}

export async function insertSupabaseData(
  table: string,
  data: any,
): Promise<SupabaseResponse> {
  try {
    const jsonData = JSON.stringify(data);
    const response = await backend.insert_to_supabase(table, jsonData);
    if ("Ok" in response) {
      const result = response.Ok;

      // Handle Candid optional types: [] | [T] where [] means None and [T] means Some(T)
      const data = result.data.length > 0 ? result.data[0] : null;
      const error = result.error.length > 0 ? result.error[0] : undefined;

      if (data) {
        try {
          const parsedData = JSON.parse(data);
          return { data: parsedData, error };
        } catch {
          // If parsing fails, return as string
          return { data, error };
        }
      }
      return { data: null, error };
    } else {
      return { error: response.Err };
    }
  } catch (error) {
    console.error("Failed to insert to Supabase:", error);
    return { error: "Failed to insert data" };
  }
}
