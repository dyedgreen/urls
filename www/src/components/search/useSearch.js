import { useEffect, useState } from "preact/hooks";

function getUrlSearch() {
  let search = window.location.search.replace("?", "").split("&").map((pair) =>
    pair.split("=")
  );
  for (const [key, value] of search) {
    if (key === "q") {
      return decodeURIComponent(value);
    }
  }
  return "";
}

function setUrlSearch(query) {
  window.history.replaceState(null, "", `?q=${encodeURIComponent(query)}`);
}

export default function useSearch(debounce = 500) {
  const [search, setSearch] = useState(getUrlSearch());
  const [immediate, setImmediate] = useState(search);

  useEffect(() => {
    const id = setTimeout(() => {
      setSearch(immediate);
      setUrlSearch(immediate);
    }, debounce);
    return () => clearTimeout(id);
  }, [immediate]);

  return [immediate, search, setImmediate];
}
