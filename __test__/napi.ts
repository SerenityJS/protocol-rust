export function compareNativeObjects(obj1: object, obj2: object): boolean {
  const keys1 = getPropertyKeysFromNativeObject(obj1);
  const keys2 = getPropertyKeysFromNativeObject(obj2);

  if (keys1.length !== keys2.length) {
    return false;
  }

  for (const key of keys1) {
    const value1 = obj1[key];
    const value2 = obj2[key];

    if (!compareValues(value1, value2)) {
      return false;
    }
  }

  return true;
}

function compareValues(value1: any, value2: any): boolean {
  if (Array.isArray(value1) && Array.isArray(value2)) {
    return compareArrays(value1, value2);
  } else if (typeof value1 === 'object' && typeof value2 === 'object') {
    return compareNativeObjects(value1, value2);
  } else {
    return value1 === value2;
  }
}

function compareArrays(arr1: any[], arr2: any[]): boolean {
  if (arr1.length !== arr2.length) {
    return false;
  }

  for (let i = 0; i < arr1.length; i++) {
    if (!compareValues(arr1[i], arr2[i])) {
      return false;
    }
  }

  return true;
}

export function getPropertyKeysFromNativeObject(obj: object): string[] {
  const keys = Object.getOwnPropertyNames(Object.getPrototypeOf(obj));
  return keys.filter((k) => typeof obj[k] !== 'function');
}
