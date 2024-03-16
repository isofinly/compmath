"use client";
import { useState } from "react";
import { Button } from "@nextui-org/react";

const LinearEquationPage = () => {
  const [solution, setSolution] = useState<any>("");
  const [isOpen, setIsOpen] = useState(false);
  const [error, setError] = useState("");

  const handleOpen = () => {
    setIsOpen(true);
  };

  const handleClose = () => {
    setIsOpen(false);
  };

  const handleSubmitString = async (
    event: React.FormEvent<HTMLFormElement>
  ) => {
    event.preventDefault();

    setSolution({
      iterations: 0,
      calculated_integral: 0,
      err: "",
    });
    try {
      const { lower_bound, upper_bound, function_id, error, method_id } =
        formData;
      const response = await fetch("http://127.0.0.1:8000/integration/string", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          function_id,
          error,
          method_id,
          lower_bound,
          upper_bound,
        }),
      });
      const data = await response.json();

      if (data.error) {
        handleOpen();
        setError(data.error);
        return;
      }
      setSolution(data);
    } catch (error) {
      console.error(error);
      handleOpen();
      setError(`Error while processing: ${error}`);
    }
  };

  const [formData, setFormData] = useState({
    function_id: 0,
    interval: [],
    lower_bound: -1,
    upper_bound: 1,
    error: 0,
    method_id: 0,
  });

  const handleChange = (e: any) => {
    const { name, value } = e.target;

    setFormData({
      ...formData,
      [name]: Number(value),
    });
  };

  const handleSubmitFile = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    setSolution({
      iterations: 0,
      calculated_integral: 0,
      err: "",
    });
    try {
      const formData = new FormData(event.currentTarget);
      const response = await fetch("http://127.0.0.1:8000/integration/file", {
        method: "POST",
        body: formData,
      });
      const data = await response.json();

      if (data.error) {
        handleOpen();
        setError(data.error);
        return;
      }
      setSolution(data);
    } catch (error) {
      console.error(error);
      handleOpen();
      setError(`Error while uploading: ${error}`);
    }
  };

  return (
    <div className="container mx-auto space-y-12 py-8">
      <div className="border-gray-900/10 border-b-2 pb-12">
        <h1 className="text-3xl font-semibold leading-7 text-gray-900">
          Лабораторная работа
        </h1>
        <p className="mt-1 text-md leading-6 text-gray-600 mb-6">
          «Численное интегрирование»
        </p>

        <div className="col-span-full mb-6">
          <div className="mt-2">
            <form
              onSubmit={handleSubmitString}
              className="grid grid-cols-2 gap-y-4"
            >
              <div className="grid grid-cols-2 gap-y-4 gap-x-4 col-span-2 text-md font-medium leading-6 text-gray-900 items-center">
                <label className="grid text-md font-medium leading-6 text-gray-900">
                  Equation ID:
                  <input
                    className="block w-50 rounded-md border-0 px-2 py-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                    type="number"
                    name="function_id"
                    min={0}
                    max={3}
                    value={formData.function_id}
                    onChange={handleChange}
                  />
                </label>
                <p className="grid col-span-1 text-md font-medium leading-6 text-gray-900">
                  0. x.powi(3) - 3.0 * x.powi(2) + 7.0 * x - 10.0 <br />
                  1. x.sin() <br />
                  2. x <br />
                  3. x / (1.0 + x.powi(2)).sqrt() <br />
                  4. 1.0 / x <br />
                  5. 1.0 / x.sqrt()
                </p>
              </div>

              <label className="grid col-span-2 text-md font-medium leading-6 text-gray-900">
                Нижняя граница
                <input
                  className="block w-50 rounded-md border-0 px-2 py-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                  type="number"
                  name="lower_bound"
                  value={formData.lower_bound}
                  onChange={handleChange}
                />
              </label>

              <label className="grid col-span-2 text-md font-medium leading-6 text-gray-900">
                Верхняя граница
                <input
                  className="block w-50 rounded-md border-0 px-2 py-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                  type="number"
                  name="upper_bound"
                  value={formData.upper_bound}
                  onChange={handleChange}
                />
              </label>

              <label className="grid col-span-2 text-md font-medium leading-6 text-gray-900">
                Calculation error:
                <input
                  className="block w-50 rounded-md border-0 px-2 py-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                  type="number"
                  name="error"
                  step={0.01}
                  value={formData.error}
                  onChange={handleChange}
                />
              </label>

              <div className="grid grid-cols-2 gap-y-4 gap-x-4 col-span-2 text-md font-medium leading-6 text-gray-900 items-center">
                <label className="grid text-md font-medium leading-6 text-gray-900">
                  Method ID:
                  <input
                    className="block w-50 rounded-md border-0 px-2 py-2 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                    type="number"
                    name="method_id"
                    min={0}
                    max={3}
                    value={formData.method_id}
                    onChange={handleChange}
                  />
                </label>
                <p className="grid col-span-1 text-md font-medium leading-6 text-gray-900">
                  0. LeftRectangles <br />
                  1. RightRectangles <br />
                  2. MiddleRectangles <br />
                  3. Trapezoid <br />
                  4. Simpson
                </p>
              </div>

              <div className="mt-2 flex items-center justify-end gap-x-6">
                <Button
                  type="submit"
                  className="rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                >
                  Рассчитать
                </Button>
              </div>
            </form>
          </div>
        </div>

        {isOpen && (
          <div className="fixed inset-0 flex items-center justify-center z-50">
            <div className="bg-gray-800 bg-opacity-75 absolute inset-0"></div>
            <div className="relative bg-white p-8 rounded-lg shadow-lg">
              <button
                className="absolute top-0 right-0  m-2"
                onClick={handleClose}
              >
                <svg
                  className="w-4 h-4"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                  xmlns="http://www.w3.org/2000/svg"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth="2"
                    d="M6 18L18 6M6 6l12 12"
                  ></path>
                </svg>
              </button>
              <p>{error}</p>
            </div>
          </div>
        )}

        <div className="col-span-full mb-6">
          <label className="block text-md font-medium leading-6 text-gray-900">
            Ручной ввод параметров
          </label>
          <div className="mt-2">
            <form onSubmit={handleSubmitFile} encType="multipart/form-data">
              <input
                name="file"
                type="file"
                className="block w-full rounded-md border-0 py-1.5 px-3 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
              />
              <div className="mt-2 flex items-center justify-end gap-x-6">
                <Button
                  type="submit"
                  className="rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                >
                  Рассчитать
                </Button>
              </div>
            </form>
          </div>
        </div>

        <div className="border-t border-gray-900/10 pb-12">
          <h1 className="text-2xl font-semibold leading-7 text-gray-900 mt-5">
            Решение
          </h1>
          {solution && solution.error && (
            <div
              className="mt-10 grid-cols-2 gap-x-6 gap-y-8 justify-items-center justify-center bg-yellow-100 border-l-4 border-yellow-500 text-yellow-700 p-4"
              role="alert"
            >
              <p className="font-bold">Внимание!</p>
              <p>{solution.error}</p>
            </div>
          )}
          <div className="mt-10 grid grid-cols-2 gap-x-6 gap-y-8 justify-items-center justify-center">
            <div className="col-span-2 w-full">
              <label className="block text-md font-medium leading-6 text-gray-900">
                Количество разбиений
              </label>
              <div className="py-2">
                <input
                  type="text"
                  name="iter"
                  id="iter"
                  value={solution ? solution.iterations : ""}
                  disabled
                  className="px-2 block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                />
              </div>
            </div>

            <div className="col-span-2 w-full">
              <label className="block text-md font-medium leading-6 text-gray-900">
                Значение интеграла
              </label>
              <div className="py-2">
                <input
                  id="mtrx"
                  value={solution ? solution.integral_value : ""}
                  disabled
                  className="px-2 block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                />
              </div>
            </div>
          </div>
        </div>

        <div id="gd" className="sm:col-span-6 col-span-1"></div>
      </div>
    </div>
  );
};

export default LinearEquationPage;
