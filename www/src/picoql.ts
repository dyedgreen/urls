import { useState, useEffect } from "preact/hooks";

function fetchQuery(query, variables) {
  const xsrfToken = window.__xsrf_token;
  return fetch("/graphql", {
    method: "POST",
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
      "X-XSRF-Token": xsrfToken,
    },
    body: JSON.stringify({ query, variables }),
  })
  .then(res => res.json());
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
    });
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

export function useMutation(mutation) {
  const [inFlight, setInFlight] = useState(false);

  const commit = async (vars) => {
    let { data, errors } = await fetchQuery(mutation, vars);
    if (errors != null)
      throw errors;
    return data;
  };

  return { commit, inFlight };
}
