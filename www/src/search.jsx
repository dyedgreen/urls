import { render, h } from "preact";
import { useState, useEffect } from "preact/hooks";
import { graphql, useQuery } from "picoql";

import ActivityIndicator from "@app/ActivityIndicator";
import ErrorBoundary from "@app/ErrorBoundary";
import BigInput from "@app/search/BigInput";
import SearchResult from "@app/search/SearchResult";

import useSearch from "@app/search/useSearch";

function Search() {
  const [search, searchDebounced, setSearch] = useSearch();

  const { data, loading, refetch } = useQuery(graphql`
    query SearchQuery($query: String!) {
      search(query: $query) {
        id
        results(first: 20) {
          edges {
            node {
              id
              url
              title
              slug
              upvoteCount
            }
          }
        }
      }
    }
  `, { query: searchDebounced });
  useEffect(() => refetch({ query: searchDebounced }), [searchDebounced]);

  const hasResults = data?.search?.results?.edges?.length > 0;
  return (
    <div class="w-full flex flex-col items-center">
      <BigInput placeholder="Search ..." value={search} onChange={setSearch} loading={loading} style="mb-2" />
      {
        hasResults ?
        data.search.results.edges.map(({node}) => {
          return <SearchResult {...node} />
        }) :
        <h3 class="text-gray-500 my-4">No matching results found</h3>
      }
    </div>
  );
}

render(<ErrorBoundary><Search /></ErrorBoundary>, document.getElementById("search"));
