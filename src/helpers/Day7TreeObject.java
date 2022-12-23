package helpers;

import days.Day7;

import java.util.ArrayList;

public class Day7TreeObject {

    public String name;
    public Day7TreeObject parent;
    public boolean isDir;
    public ArrayList<Day7TreeObject> children;

    public int size; // this includes all the children

    public Day7TreeObject (String name, Day7TreeObject parent, Boolean isDir, int size) {
        this.name = name;
        this.parent = parent;
        this.isDir = isDir;
        this.size = size;

        children = new ArrayList<>();
    }

    public boolean addChild(Day7TreeObject child) {
        children.add(child);
        return true;
    }

    public int getSize() {
        if (isDir) {
            int sum = 0;
            for (Day7TreeObject child: children) {
                sum += child.getSize();
            }
            size = sum;
            return size;
        } else {
            return size;
        }

    }

}
