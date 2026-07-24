import { useEffect, useRef } from "react";
import { listen, type EventCallback } from "@tauri-apps/api/event";

export function useTauriEvent<T>(eventName: string, handler: EventCallback<T>): void {
  const handlerRef = useRef(handler);
  handlerRef.current = handler;

  useEffect(() => {
    let mounted = true;
    let removeListener: (() => void) | undefined;

    void listen<T>(eventName, (event) => {
      handlerRef.current(event);
    }).then((unlisten) => {
      if (mounted) {
        removeListener = unlisten;
      } else {
        unlisten();
      }
    });

    return () => {
      mounted = false;
      removeListener?.();
    };
  }, [eventName]);
}
