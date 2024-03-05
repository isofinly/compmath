import React, { useEffect, useRef } from "react";

const SystemChartComponent: React.FC<{
  formData: { eq_id: number };
  solution: { x: number; y: number };
}> = ({ formData, solution }) => {
  const calculatorRef = useRef<any | null>(null);

  useEffect(() => {
    const timeoutId = setTimeout(() => {
      if (!calculatorRef.current) {
        const elt = document.getElementById("calculator");
        calculatorRef.current = Desmos.GraphingCalculator(elt);
      }

      const calculator = calculatorRef.current;

      switch (formData?.eq_id) {
        case 0: {
          calculator.setExpression({ id: "graph1", latex: "x^2 + y^2 = 4" });
          calculator.setExpression({ id: "graph2", latex: "y=3x^2" });
          calculator.setExpression({
            id: "graph3",
            latex: `0 - ${solution.x}^2 + ${solution.y}^2 - 4`,
          });
          break;
        }
        case 1: {
          // 2.0 * y - (x+1.0).cos(), x + y.sin() + 0.4
          calculator.setExpression({
            id: "graph1",
            latex: "0=x^2 + x - y^2 - 0.15",
          });
          calculator.setExpression({
            id: "graph2",
            latex: "0=x^2 - y + y^2 + 0.17",
          });
          calculator.setExpression({
            id: "graph3",
            latex: `0 - (${solution?.x}^2 + ${solution?.x} - ${solution?.y}^2 - 0.15)`,
          });
          break;
        }
        case 2: {
          // 2.0 * y - (x+1.0).cos(), x + y.sin() + 0.4
          calculator.setExpression({
            id: "graph1",
            latex: "0=2 * y - \\cos(x+1)",
          });
          calculator.setExpression({
            id: "graph2",
            latex: "0=x + \\sin(y) + 0.4",
          });
          calculator.setExpression({
            id: "graph3",
            latex: `0 - (2 * ${solution?.y} - \\cos(${solution?.x} + 1))`,
          });
          break;
        }
        default:
          break;
      }
    }, 1000);
    return () => clearTimeout(timeoutId);
  }, [formData?.eq_id, solution]);

  return <div id="calculator" style={{ width: "100%", height: "400px" }}></div>;
};

export default SystemChartComponent;
