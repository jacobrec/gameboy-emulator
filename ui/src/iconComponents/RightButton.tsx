import * as React from "react";

function SvgRightButton(props: React.SVGProps<SVGSVGElement>) {
  return (
    <svg
      data-name="Layer 1"
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 127 127"
      width="1em"
      height="1em"
      {...props}
    >
      <rect
        x={1.09}
        y={0.77}
        width={126}
        height={126}
        rx={7}
        transform="rotate(90 63.93 63.34)"
        strokeMiterlimit={10}
        fill="#999"
        stroke="#999"
      />
      <path
        stroke="#000"
        strokeMiterlimit={10}
        d="M112.27 65.06L59.01 15.74H14.73v95.52h44.28l53.26-46.2z"
      />
    </svg>
  );
}

export default SvgRightButton;
