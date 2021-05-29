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
  const [refetchID, setRefetchID] = useState(0);
  const [data, setData] = useState(null);
  const [loading, setLoading] = useState(true);
  const [errors, setErrors] = useState(null);

  if (errors != null)
    throw errors;

  const refetch = () => {
    setLoading(true);
    setRefetchID(refetchID + 1);
  };

  useEffect(() => {
    fetchQuery(query, args)
    .then(({data, errors}) => {
      setLoading(false);
      if (errors != null) {
        setErrors(errors);
      } else {
        setData(data);
      }
    })
    .catch(err => setErrors(err));
  }, [query, args, refetchID]);

  return {
    data,
    loading,
    refetch,
  }
}

export function useMutation(mutation, { onCommit, onError }) {
  const [inFlight, setInFlight] = useState(false);
  const [errors, setErrors] = useState(undefined);

  if (errors !== undefined)
    throw errors;

  const commit = (vars) => {
    setInFlight(true);
    fetchQuery(mutation, vars)
    .then(({ data, errors }) => {
      setInFlight(false);
      if (errors != null) {
        if (typeof onError === "function") {
          onError(errors);
        } else {
          setErrors(errors)
        }
      } else {
        onCommit(data);
      }
    })
    .catch(err => setErrors(err));
  };

  return { commit, inFlight };
}
