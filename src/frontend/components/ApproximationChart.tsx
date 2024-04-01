import React, { useEffect, useRef } from "react";

const ApproximationChartComponent: React.FC<{
  solution: {
    function: string;
    coefficients: number[];
    differences: number[];
    epsilon_values: number[];
    pearson_correlation: number;
    phi_values: number[];
    standard_deviation: number;
    data_points: number[] | any[];
  };
}> = ({ solution }) => {
  const calculatorRef = useRef<any | null>(null);

  useEffect(() => {
    const timeoutId = setTimeout(() => {
      if (!calculatorRef.current) {
        const elt = document.getElementById("calculator");
        calculatorRef.current = Desmos.GraphingCalculator(elt);
      }

      const calculator = calculatorRef.current;
      if (!solution) return;
      calculator.setExpression({ id: "graph1", latex: solution.function });
      solution.data_points.forEach((point, index) => {
        calculator.setExpression({
          id: `point${index}`,
          latex: `(${solution.data_points[index].x}, ${point.phi_x})`,
        });
      })
    }, 1000);
    return () => clearTimeout(timeoutId);
  }, [solution]);

  return <div id="calculator" style={{ width: "100%", height: "400px" }}></div>;
};
export default ApproximationChartComponent;
