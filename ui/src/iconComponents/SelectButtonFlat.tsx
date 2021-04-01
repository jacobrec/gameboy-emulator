import * as React from "react";

function SvgSelectButtonFlat(props: React.SVGProps<SVGSVGElement>) {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 95.04 36.27"
      width="1em"
      height="1em"
      {...props}
    >
      <path
        d="M87.17 17.8l-79.88.09c-3.88 0-6.94-3.66-6.77-8.13v-1C.69 4.29 3.99.59 7.87.59L87.75.5c3.88 0 6.94 3.65 6.79 8.13v1c-.19 4.5-3.49 8.17-7.37 8.17z"
        fill="gray"
        stroke="gray"
        strokeMiterlimit={10}
      />
      <text
        transform="rotate(-.08 22972.34 -15826.25)"
        fontSize={16.35}
        fontFamily="AvantGarde-Medium,ITC Avant Garde Gothic"
        fontWeight={500}
      >
        {"SELECT"}
      </text>
    </svg>
  );
}

export default SvgSelectButtonFlat;
