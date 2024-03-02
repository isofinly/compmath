import React, { useEffect, useRef } from "react";

const CalculatorComponent: React.FC<{
  formData: { method_id: number; eq_id: number };
}> = ({ formData }) => {
  const calculatorRef = useRef<Desmos.GraphingCalculator | null>(null);

  useEffect(() => {
    if (!calculatorRef.current) {
      const elt = document.getElementById("calculator");
      calculatorRef.current = Desmos.GraphingCalculator(elt);
    }

    const calculator = calculatorRef.current;

    switch (formData.eq_id) {
      case 0: {
        calculator.setExpression({ id: "graph1", latex: "y=x^2 + y^2 - 4" });
        calculator.setExpression({ id: "graph2", latex: "y=3x^2" });
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
        break;
      }
      default:
        break;
    }
  }, [formData.method_id, formData.eq_id]);

  return <div id="calculator" style={{ width: "100%", height: "400px" }}></div>;
};

export default CalculatorComponent;
