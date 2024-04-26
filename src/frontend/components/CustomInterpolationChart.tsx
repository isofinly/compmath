import React, { useEffect, useRef } from "react";

const SingleChartComponent: React.FC<{
  formData: { equation: string, nodes: any};
}> = ({ formData }) => {
  const calculatorRef = useRef<any | null>(null);

  useEffect(() => {
    const timeoutId = setTimeout(() => {
      if (!calculatorRef.current) {
        const elt = document.getElementById("calculator");
        calculatorRef.current = Desmos.GraphingCalculator(elt);
      }

      const calculator = calculatorRef.current;

      calculator.setBlank();

      calculator.setExpression({
        id: "graph1",
        latex: formData.equation,
      });
      
      const pairs = formData.nodes[0].map((x: any, i: string | number) => ({ x, y: formData.nodes[1][i] }));

      pairs.forEach((pair: { x: any; y: any; }, index: any) => {
        calculator.setExpression({
          id: `point${index}`,
          latex: `(${pair.x}, ${pair.y})`,
        });
      });
    
    }, 1000);
    return () => clearTimeout(timeoutId);
  }, [formData]);

  return <div id="calculator" style={{ width: "100%", height: "400px" }}></div>;
};

export default SingleChartComponent;
