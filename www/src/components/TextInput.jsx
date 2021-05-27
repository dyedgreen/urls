import { h } from "preact";
import { useState } from "preact/hooks";

export default function TextInput({ value, onChange, label, placeholder, style }) {
  const [inputID] = useState(`input-id-${Math.floor(Math.random() * 10000)}`);
  return <div class={`w-full ${style ?? ""}`}>
    <label
      class="text-gray-500 italic"
      for={inputID}
    >
      {label}
    </label>
    <input
      class="w-full p-2 text-md rounded-md bg-gray-200 dark:bg-gray-600 dark:text-white"
      id={inputID}
      type="text"
      value={value}
      onInput={e => typeof onChange === "function" && onChange(e.target.value)}
      placeholder={placeholder}
    />
  </div>;
}
