import { h } from "preact";
import { useState } from "preact/hooks";

const ICON_UP = <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 15l7-7 7 7" />
</svg>;
const ICON_DOWN = <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
</svg>;

export default function({ title, children, initiallyExpanded = true }) {
  const [expanded, setExpanded] = useState(initiallyExpanded);

  return (
    <div class="w-full mt-2 rounded-md bg-gray-100 dark:bg-gray-700">
      <button
        class="w-full p-2 flex justify-between items-center rounded-md hover:bg-gray-200 dark:hover:bg-gray-600 dark:text-white"
        onClick={() => setExpanded(!expanded)}
      >
        <h1 class="text-lg font-semibold">{title}</h1>
        {expanded ? ICON_UP : ICON_DOWN}
      </button>
      <div class="w-full px-2 pb-2" style={{ display: expanded ? undefined : "none" }}>
        {children}
      </div>
    </div>
  );
}
