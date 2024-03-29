import React, { useEffect, useRef } from "react";

const SingleChartComponent: React.FC<{
  formData: { eq_id: number };
}> = ({ formData }) => {
  const calculatorRef = useRef<any | null>(null);

  useEffect(() => {
    const timeoutId = setTimeout(() => {
      if (!calculatorRef.current) {
        const elt = document.getElementById("calculator");
        calculatorRef.current = Desmos.GraphingCalculator(elt);
      }

      const calculator = calculatorRef.current;

      switch (formData.eq_id) {
        case 0: {
          // 2.0 * y - (x+1.0).cos(), x + y.sin() + 0.4
          calculator.setExpression({
            id: "graph1",
            latex: "1.62x^3 - 8.15x^2 + 4.39x + 4.29",
          });
          break;
        }
        case 1: {
          // 2.0 * y - (x+1.0).cos(), x + y.sin() + 0.4
          calculator.setExpression({
            id: "graph1",
            latex: "x^3 - x + 4 ",
          });
          break;
        }
        case 2: {
          // 2.0 * y - (x+1.0).cos(), x + y.sin() + 0.4
          calculator.setExpression({
            id: "graph1",
            latex: "\\exp(x) - 5",
          });
          break;
        }
        case 3: {
          // 2.0 * y - (x+1.0).cos(), x + y.sin() + 0.4
          calculator.setExpression({
            id: "graph1",
            latex: "\\sin(2*x) + \\pi/4",
          });
          break;
        }
        default:
          break;
      }
    }, 1000);
    return () => clearTimeout(timeoutId);
  }, [formData.eq_id]);

  return <div id="calculator" style={{ width: "100%", height: "400px" }}></div>;
};

export default SingleChartComponent;
