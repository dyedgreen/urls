import { h } from "preact";

export default function ActivityIndicator({ size = "small", style = "" }) {
  const classes = size === "large" ? "h-8 w-8" : "h-4 w-4";
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class={`${classes} ${style ?? ""}`} viewBox="0 0 24 24" stroke="none" fill="currentColor">
      <rect x="10" y="0" width="4" height="8" rx="2" opacity="0">
        <animate
          attributeName="opacity"
          from="1"
          to="0"
          dur="1s"
          begin="0s"
          repeatCount="indefinite"
        />
      </rect>
      <rect x="-2" y="-4" width="4" height="8" rx="2" transform="translate(17.5, 6.5) rotate(45)" opacity="0">
        <animate
          attributeName="opacity"
          from="1"
          to="0"
          dur="1s"
          begin="0.125s"
          repeatCount="indefinite"
        />
      </rect>
      <rect x="16" y="10" width="8" height="4" rx="2" opacity="0">
        <animate
          attributeName="opacity"
          from="1"
          to="0"
          dur="1s"
          begin="0.25s"
          repeatCount="indefinite"
        />
      </rect>
      <rect x="-2" y="-4" width="4" height="8" rx="2" transform="translate(17.5, 17.5) rotate(-45)" opacity="0">
        <animate
          attributeName="opacity"
          from="1"
          to="0"
          dur="1s"
          begin="0.375s"
          repeatCount="indefinite"
        />
      </rect>
      <rect x="10" y="16" width="4" height="8" rx="2" opacity="0">
        <animate
          attributeName="opacity"
          from="1"
          to="0"
          dur="1s"
          begin="0.5s"
          repeatCount="indefinite"
        />
      </rect>
      <rect x="-2" y="-4" width="4" height="8" rx="2" transform="translate(6.5, 17.5) rotate(45)" opacity="0">
        <animate
          attributeName="opacity"
          from="1"
          to="0"
          dur="1s"
          begin="0.625s"
          repeatCount="indefinite"
        />
      </rect>
      <rect x="0" y="10" width="8" height="4" rx="2" opacity="0">
        <animate
          attributeName="opacity"
          from="1"
          to="0"
          dur="1s"
          begin="0.75s"
          repeatCount="indefinite"
        />
      </rect>
      <rect x="-2" y="-4" width="4" height="8" rx="2" transform="translate(6.5, 6.5) rotate(-45)" opacity="0">
        <animate
          attributeName="opacity"
          from="1"
          to="0"
          dur="1s"
          begin="0.875s"
          repeatCount="indefinite"
        />
      </rect>
    </svg>
  );
}
