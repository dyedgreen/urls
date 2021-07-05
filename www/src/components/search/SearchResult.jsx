import { h } from "preact";

const UP_ICON = <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
  <path fill-rule="evenodd" d="M5.293 9.707a1 1 0 010-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 01-1.414 1.414L11 7.414V15a1 1 0 11-2 0V7.414L6.707 9.707a1 1 0 01-1.414 0z" clip-rule="evenodd" />
</svg>;

export default function SearchResult({ id, title, url, upvoteCount }) {
  return <a
    class="block w-full flex p-2 rounded hover:bg-gray-200 dark:hover:bg-gray-700"
    href={`/comments/${id}`}
  >
    <div class="flex items-center text-blue-500 mr-4">
      {UP_ICON}
      <span class="text-sm leading-none">{upvoteCount}</span>
    </div>
    <div class="flex-grow">
      <h1 class="mb-1 text-md font-semibold leading-none">{title}</h1>
      <h2 class="text-sm text-gray-500 leading-none break-all">{url}</h2>
    </div>
  </a>;
}
