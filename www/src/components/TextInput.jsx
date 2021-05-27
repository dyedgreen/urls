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
      class="w-full bg-gray-200 p-2 text-md rounded-md"
      id={inputID}
      type="text"
      value={value}
      onInput={e => typeof onChange === "function" && onChange(e.target.value)}
      placeholder={placeholder}
    />
  </div>;
}
