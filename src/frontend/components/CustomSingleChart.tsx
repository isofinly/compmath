import React, { useMemo } from "react";
import { Line } from "react-chartjs-2";
import "chart.js/auto";

function generateArray(start: number, finish: number, step: number) {
  let result = [];
  for (let i = start; i <= finish; i += step) {
      result.push(i.toFixed(2));
  }
  return result;
}

const CustomChart: React.FC<any> = ({start, finish, index}) => {
  const data = useMemo(() => {
    const labels = generateArray(start-(finish-start*0.3), finish+(finish-start*0.3), 0.3);
    const datasets = [
      {
        label: "f(x) = 1.62x^3 - 8.15x^2 + 4.39x + 4.29",
        function: (x: any) =>
          1.62 * Math.pow(x, 3) - 8.15 * Math.pow(x, 2) + 4.39 * x + 4.29,
        borderColor: "rgba(75, 192, 192, 1)",
        fill: false,
      },
      {
        label: "f(x) = x^3 - x + 4",
        function: (x: number) => Math.pow(x, 3) - x + 4,
        borderColor: "rgba(153, 102, 255, 1)",
        fill: false,
      },
      {
        label: "f(x) = e^x - 5",
        function: (x: number) => Math.exp(x) - 5,
        borderColor: "rgba(255, 206, 86, 1)",
        fill: false,
      },
      {
        label: "f(x) = sin(2x) + π/4",
        function: (x: number) => Math.sin(2 * x) + Math.PI / 4,
        borderColor: "rgba(255, 99, 132, 1)",
        fill: false,
      },
    ].map((dataset, i) => 
      i === index ? {
        ...dataset,
        data: labels.map((label) => dataset.function(label)),
      } : null
    ).filter(Boolean);

    return {
      labels,
      datasets,
    };
  }, [start, finish, index]);

  const options = {
    cubicInterpolationMode: 'monotone',
    x: {
      display: true,
      title: {
        display: true
      }
    },
    y: {
      display: true,
      title: {
        display: true,
        text: 'Value'
      },
    },
  };

  return <Line data={data} options={options} />;
};

export default CustomChart;