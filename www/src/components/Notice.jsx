import { h } from "preact";

const ICON_NOTICE = <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z" />
</svg>;
const ICON_ERROR = <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
</svg>;

export default function Notice({ type = "info", message, style }) {
  let classes = `flex align-start p-3 rounded-md block font-semibold ${style ?? ""}`;
  let icon;
  switch (type) {
    case "error":
      classes += " bg-red-200 text-red-800";
      icon = ICON_ERROR;
      break;
    case "warning":
      classes += " bg-yellow-200 text-yellow-800";
      icon = ICON_ERROR;
      break;
    case "info":
    default:
      classes += " bg-blue-200 text-blue-800";
      icon = ICON_NOTICE;
  }
  return <div class={classes}>
      <div class="mr-2">{icon}</div>
      <p class="leading-snug">{message}</p>
    </div>
}
