import * as React from "react";

function SvgStartButtonAngled(props: React.SVGProps<SVGSVGElement>) {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 96.14 48.08"
      width="1em"
      height="1em"
      {...props}
    >
      <path
        d="M90.64 16.33L10.78 47.09a7.3 7.3 0 01-9.54-4.53l-.32-.91a8 8 0 014.57-10L85.36.99a7.29 7.29 0 019.54 4.52l.32.91a8 8 0 01-4.58 9.91z"
        fill="gray"
        stroke="gray"
        strokeMiterlimit={10}
        data-name="Layer 1"
      />
      <text
        transform="matrix(.93 -.36 .36 .93 57.67 41.69)"
        fontSize={14}
        fontFamily="AvantGarde-Medium,ITC Avant Garde Gothic"
        fontWeight={500}
      >
        {"S"}
        <tspan x={6.99} y={0} letterSpacing="-.02em">
          {"T"}
        </tspan>
        <tspan x={12.33} y={0}>
          {"A"}
        </tspan>
        <tspan x={21.91} y={0} letterSpacing=".01em">
          {"R"}
        </tspan>
        <tspan x={30.44} y={0} letterSpacing={0}>
          {"T"}
        </tspan>
      </text>
    </svg>
  );
}

export default SvgStartButtonAngled;
