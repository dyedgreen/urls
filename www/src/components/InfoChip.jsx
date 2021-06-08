import { h } from "preact";

export default function InfoChip({ icon, text }) {
  return (
    <div class="flex items-center">
      {icon} {text}
    </div>
  );
}
