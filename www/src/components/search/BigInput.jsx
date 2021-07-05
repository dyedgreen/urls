import { h } from "preact";

import ActivityIndicator from "@app/ActivityIndicator";

export default function BigInput({ value, onChange, placeholder, loading, style }) {
  return <div class={`
    flex space-between w-full p-4 rounded-md
    bg-gray-200 dark:bg-gray-600
    text-black dark:text-white
    ${style ?? ""}
  `}>
    <input
      class="w-4 h-8 flex-grow text-xl font-semibold bg-gray-200 dark:bg-gray-600"
      type="text"
      value={value}
      onInput={e => typeof onChange === "function" && onChange(e.target.value)}
      placeholder={placeholder}
    />
    {loading && <ActivityIndicator size="large" />}
  </div>;
}
