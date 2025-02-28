import axios, { AxiosRequestConfig } from "axios";
import { useEffect, useState } from "react";

interface FetchResult<T> {
  data: T | null;
  loading: boolean;
  error: Error | null;
}

const useFetch = <T>(
  url: string,
  options?: AxiosRequestConfig,
): FetchResult<T> => {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    const controller = new AbortController(); // To cancel request on unmount
    const fetchData = async () => {
      setLoading(true);
      try {
        const response = await axios.get<T>(url, {
          ...options,
          signal: controller.signal, // Attach abort signal
        });
        setData(response.data);
        setError(null);
      } catch (err) {
        if (axios.isCancel(err)) {
          return; // Ignore abort errors
        }
        setError(err as Error);
      } finally {
        setLoading(false);
      }
    };

    fetchData();

    return () => controller.abort();
  }, [url]);

  return { data, loading, error };
};

export default useFetch;
