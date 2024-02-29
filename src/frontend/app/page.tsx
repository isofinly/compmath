'use client';
import {
  Navbar,
  NavbarBrand,
  NavbarContent,
  NavbarItem,
  Link,
  Button,
  Spacer
} from "@nextui-org/react";
import React, { useState, useEffect } from 'react';

const CountdownTimer = () => {
    const [timeLeft, setTimeLeft] = useState(calculateTimeLeft());
  
    useEffect(() => {
      const timer = setTimeout(() => {
        setTimeLeft(calculateTimeLeft());
      }, 1000);
  
      return () => clearTimeout(timer);
    });
  
    function calculateTimeLeft() {
      const now = new Date();
      const targetTime = new Date(now);
      targetTime.setHours(24, 0, 0, 0); // Set target time to today 00:00:00
  
      let difference = targetTime - now;
  
      if (difference < 0) {
        const tomorrow = new Date(now);
        tomorrow.setDate(now.getDate() + 1);
        tomorrow.setHours(0, 0, 0, 0); // Set target time to tomorrow 00:00:00
        difference = tomorrow - now;
      }
  
      const hours = Math.floor((difference / (1000 * 60 * 60)) % 24);
      const minutes = Math.floor((difference / 1000 / 60) % 60);
      const seconds = Math.floor((difference / 1000) % 60);
  
      return {
        hours: hours < 10 ? '0' + hours : hours,
        minutes: minutes < 10 ? '0' + minutes : minutes,
        seconds: seconds < 10 ? '0' + seconds : seconds,
      };
    }
  
    return (
      <div>
        <h1 className="text-5xl font-bold underline flex justify-center ">ВЗРЫВ ЯДЕРКИ ЧЕРЕЗ</h1>
        <Spacer y={20} />
        <div className="text-8xl font-bold flex justify-center text-red-500">
          {/* <span>{timeLeft.hours}</span>:<span>{timeLeft.minutes}</span>:<span>{timeLeft.seconds}</span> */}
        </div>
      </div>
    );
  };

export default CountdownTimer;