import React, { useEffect, useRef } from "react";

const SingleChartComponent: React.FC<{
  solutionData: any;
}> = ({ solutionData }) => {
  const calculatorRef = useRef<any | null>(null);

  useEffect(() => {
    const timeoutId = setTimeout(() => {
      if (!solutionData) return;

      if (!calculatorRef.current) {
        const elt = document.getElementById("calculator");
        calculatorRef.current = Desmos.GraphingCalculator(elt);
      }

      const calculator = calculatorRef.current;
      
      calculator.setBlank();
      calculator.setExpression({
        id: `equation`,
        latex: `y = ${solutionData.equation}`,
      });
      solutionData.points.forEach((pair: number[], index: any) => {
        calculator.setExpression({
          id: `point${index}`,
          latex: `(${pair[0].toFixed(4)}, ${pair[1].toFixed(4)})`,
        });
      });
    
    }, 1000);
    return () => clearTimeout(timeoutId);
  }, [solutionData]);

  return <div id="calculator" style={{ width: "100%", height: "400px" }}></div>;
};

export default SingleChartComponent;
