import { useState, useEffect } from "preact/hooks";

async function fetchQuery(query, variables) {
  const xsrfToken = window.__xsrf_token;
  const resp = await fetch("/graphql", {
    method: "POST",
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
      "X-XSRF-Token": xsrfToken,
    },
    body: JSON.stringify({ query, variables }),
  });
  return await resp.json();
}

export function graphql([query]) {
  return query;
}

export function useQuery(query, args) {
  const [data, setData] = useState(null);
  const [loading, setLoading] = useState(true);
  const [errors, setErrors] = useState(null);

  useEffect(() => {
    fetchQuery(query, args).then(({data, errors}) => {
      if (errors != null) {
        setErrors(errors);
      } else {
        setData(data);
        setLoading(false);
      }
    })
    .catch(err => setErrors(err));
  }, [query, args]);

  useEffect(() => {
    if (errors != null) {
      throw errors;
    }
  }, [errors]);

  return {
    data,
    loading,
  }
}

export function useMutation(mutation, { onCommit, onError }) {
  const [inFlight, setInFlight] = useState(false);
  const [errors, setErrors] = useState(undefined);

  if (errors !== undefined)
    throw errors;

  const commit = (vars) => {
    (async () => {
      setInFlight(true);
      const { data, errors } = await fetchQuery(mutation, vars);
      setInFlight(false);

      if (errors != null) {
        if (typeof onError === "function") {
          onError(errors);
        } else {
          setErrors(error)
        }
      } else {
        onCommit(data);
      }
    })();
  };

  return { commit, inFlight };
}
