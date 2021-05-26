import { h } from "preact";

export default function Button({title, onClick, type = "default", style}) {
  let classes = `h-8 px-2 rounded-md font-bold ${style ?? ""}`;
  switch (type) {
    case "flat":
      classes += " bg-white text-black hover:bg-gray-100";
      break;
    case "default":
    default:
      classes += " bg-blue-600 text-white hover:bg-blue-800";
  }
  return <button
    class={classes}
    onClick={onClick}
  >
    {title}
  </button>;
}
