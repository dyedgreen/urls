import { h } from "preact";
import ActivityIndicator from "@app/ActivityIndicator";

export default function Button({
  title,
  href,
  disabled = false,
  style,
}) {
  let classes = `
    block h-8 px-2
    flex space-x-2 justify-center items-center
    rounded-md font-semibold ${style ?? ""}
    text-blue-500 dark:text-blue-300
    hover:bg-gray-200 dark:hover:bg-gray-600
  `;

  if (disabled === true) {
    classes += " opacity-80 cursor-not-allowed";
  }

  return <a class={classes} href={href}>{title}</a>;
}
