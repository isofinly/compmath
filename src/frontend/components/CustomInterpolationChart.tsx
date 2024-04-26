import React, { useEffect, useRef } from "react";

const SingleChartComponent: React.FC<{
  formData: { equation: string, nodes: any } | any;
}> = ({ formData }) => {
  const calculatorRef = useRef<any | null>(null);

  useEffect(() => {
    const timeoutId = setTimeout(() => {
      if (!formData) return
      if (!formData.newton_separated) return
      if (!formData.newton_separated.nodes) return
      if (!calculatorRef.current) {
        const elt = document.getElementById("calculator");
        calculatorRef.current = Desmos.GraphingCalculator(elt);
      }

      const calculator = calculatorRef.current;

      calculator.setBlank();

      calculator.setExpression({
        id: "graph1",
        latex: formData.newton_separated.latex_function,
      });
      
      const pairs = formData.newton_separated.nodes[0].map((x: any, i: string | number) => ({ x, y: formData.newton_separated.nodes[1][i] }));

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
