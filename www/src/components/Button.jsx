import { h } from "preact";
import ActivityIndicator from "@app/ActivityIndicator";

export default function Button({
  title,
  onClick,
  disabled = false,
  loading = false,
  type = "default",
  style,
}) {
  let classes = `h-8 px-2 flex space-x-2 justify-center items-center rounded-md font-bold ${style ?? ""}`;

  switch (type) {
    case "flat":
      classes += " bg-white text-black";
      if (!disabled)
        classes += " hover:bg-gray-200";
      break;
    case "default":
    default:
      classes += " bg-blue-500 text-white";
      if (!disabled)
        classes += " hover:bg-blue-400";
  }

  if (disabled === true) {
    classes += " opacity-80 cursor-not-allowed";
  }

  return <button class={classes} onClick={disabled === true ? undefined : onClick}>
    <div>
      {title}
    </div>
    {loading === true && <ActivityIndicator />}
  </button>;
}
