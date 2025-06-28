import { useState, useCallback } from "react";
import {
  fetchSupabaseData,
  insertSupabaseData,
} from "../services/supabaseService";

interface UseSupabaseReturn {
  fetch: (table: string, query?: string) => Promise<unknown>;
  insert: (table: string, data: unknown) => Promise<unknown>;
  loading: boolean;
  error: string | null;
}

export function useSupabase(): UseSupabaseReturn {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetch = useCallback(
    async (table: string, query?: string): Promise<unknown> => {
      setLoading(true);
      setError(null);

      try {
        const response = await fetchSupabaseData(table, query || "");
        if (response.error) {
          setError(response.error);
          return null;
        }
        return response.data;
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : "Unknown error occurred";
        setError(errorMessage);
        return null;
      } finally {
        setLoading(false);
      }
    },
    [],
  );

  const insert = useCallback(
    async (table: string, data: unknown): Promise<unknown> => {
      setLoading(true);
      setError(null);

      try {
        const response = await insertSupabaseData(table, data);
        if (response.error) {
          setError(response.error);
          return null;
        }
        return response.data;
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : "Unknown error occurred";
        setError(errorMessage);
        return null;
      } finally {
        setLoading(false);
      }
    },
    [],
  );

  return {
    fetch,
    insert,
    loading,
    error,
  };
}
