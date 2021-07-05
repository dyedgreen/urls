import { render, h } from "preact";
import { useState } from "preact/hooks";
import { graphql, useQuery } from "picoql";

import ActivityIndicator from "@app/ActivityIndicator";
import ErrorBoundary from "@app/ErrorBoundary";
import BigInput from "@app/search/BigInput";

import useSearch from "@app/search/useSearch";

function Search() {
  const [search, searchDebounced, setSearch] = useSearch();

  const { data, loading } = useQuery(graphql`
    query SearchQuery {
      viewer {
        id
        user {
          id
          name
        }
      }
    }
  `);

  return (
    <div class="w-full flex justify-center p-8">
      <div class="w-full flex flex-col items-center max-w-screen-md bg-white dark:bg-gray-800 shadow rounded-lg p-4">
        <BigInput placeholder="Search ..." value={search} onChange={setSearch} loading={false} />
      </div>
    </div>
  );
}

render(<ErrorBoundary><Search /></ErrorBoundary>, document.getElementById("search"));
