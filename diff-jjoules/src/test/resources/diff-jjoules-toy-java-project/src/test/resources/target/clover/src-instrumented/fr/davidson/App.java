/* $$ This file has been instrumented by Clover 4.4.1#2019101123313948 $$ */package fr.davidson;

import java.util.*;

/**
 * Hello world!
 */
public class App {public static class __CLR4_4_100l47116a9{public static com_atlassian_clover.CoverageRecorder R;public static com_atlassian_clover.CloverProfile[] profiles = { };@java.lang.SuppressWarnings("unchecked") public static <I, T extends I> I lambdaInc(final int i,final T l,final int si){java.lang.reflect.InvocationHandler h=new java.lang.reflect.InvocationHandler(){public java.lang.Object invoke(java.lang.Object p,java.lang.reflect.Method m,java.lang.Object[] a) throws Throwable{R.inc(i);R.inc(si);try{return m.invoke(l,a);}catch(java.lang.reflect.InvocationTargetException e){throw e.getCause()!=null?e.getCause():new RuntimeException("Clover failed to invoke instrumented lambda",e);}}};return (I)java.lang.reflect.Proxy.newProxyInstance(l.getClass().getClassLoader(),l.getClass().getInterfaces(),h);}static{com_atlassian_clover.CoverageRecorder _R=null;try{com_atlassian_clover.CloverVersionInfo.An_old_version_of_clover_is_on_your_compilation_classpath___Please_remove___Required_version_is___4_4_1();if(2019101123313948L!=com_atlassian_clover.CloverVersionInfo.getBuildStamp()){com_atlassian_clover.Clover.l("[CLOVER] WARNING: The Clover version used in instrumentation does not match the runtime version. You need to run instrumented classes against the same version of Clover that you instrumented with.");com_atlassian_clover.Clover.l("[CLOVER] WARNING: Instr=4.4.1#2019101123313948,Runtime="+com_atlassian_clover.CloverVersionInfo.getReleaseNum()+"#"+com_atlassian_clover.CloverVersionInfo.getBuildStamp());}R=com_atlassian_clover.Clover.getNullRecorder();_R=com_atlassian_clover.Clover.getNullRecorder();_R=com_atlassian_clover.Clover.getRecorder("\u002f\u0068\u006f\u006d\u0065\u002f\u0062\u0065\u006e\u006a\u0061\u006d\u0069\u006e\u002f\u0077\u006f\u0072\u006b\u0073\u0070\u0061\u0063\u0065\u002f\u0064\u0069\u0066\u0066\u002d\u0078\u006a\u006f\u0075\u006c\u0065\u0073\u002f\u0064\u0069\u0066\u0066\u002d\u006a\u006a\u006f\u0075\u006c\u0065\u0073\u002f\u0073\u0072\u0063\u002f\u0074\u0065\u0073\u0074\u002f\u0072\u0065\u0073\u006f\u0075\u0072\u0063\u0065\u0073\u002f\u0064\u0069\u0066\u0066\u002d\u006a\u006a\u006f\u0075\u006c\u0065\u0073\u002d\u0074\u006f\u0079\u002d\u006a\u0061\u0076\u0061\u002d\u0070\u0072\u006f\u006a\u0065\u0063\u0074\u002f\u0074\u0061\u0072\u0067\u0065\u0074\u002f\u0063\u006c\u006f\u0076\u0065\u0072\u002f\u0063\u006c\u006f\u0076\u0065\u0072\u002e\u0064\u0062",1654779572971L,8589935092L,26,profiles,new java.lang.String[]{"clover.distributed.coverage",null});}catch(java.lang.SecurityException e){java.lang.System.err.println("[CLOVER] FATAL ERROR: Clover could not be initialised because it has insufficient security privileges. Please consult the Clover documentation on the security policy file changes required. ("+e.getClass()+":"+e.getMessage()+")");}catch(java.lang.NoClassDefFoundError e){java.lang.System.err.println("[CLOVER] FATAL ERROR: Clover could not be initialised. Are you sure you have Clover in the runtime classpath? ("+e.getClass()+":"+e.getMessage()+")");}catch(java.lang.Throwable t){java.lang.System.err.println("[CLOVER] FATAL ERROR: Clover could not be initialised because of an unexpected error. ("+t.getClass()+":"+t.getMessage()+")");}R=_R;}}public static final com_atlassian_clover.TestNameSniffer __CLR4_4_1_TEST_NAME_SNIFFER=com_atlassian_clover.TestNameSniffer.NULL_INSTANCE;

    public static void sort(List<Integer> elementsToSort) {try{__CLR4_4_100l47116a9.R.inc(0);
        __CLR4_4_100l47116a9.R.inc(1);Collections.sort(elementsToSort);
    }finally{__CLR4_4_100l47116a9.R.flushNeeded();}}

    // source: https://rosettacode.org/wiki/Sorting_algorithms/Quicksort#Java
    public static List<Integer> quickSort(List<Integer> arr) {try{__CLR4_4_100l47116a9.R.inc(2);
        __CLR4_4_100l47116a9.R.inc(3);if ((((arr.isEmpty())&&(__CLR4_4_100l47116a9.R.iget(4)!=0|true))||(__CLR4_4_100l47116a9.R.iget(5)==0&false)))
            {__CLR4_4_100l47116a9.R.inc(6);return arr;
        }else {{
            __CLR4_4_100l47116a9.R.inc(7);Integer pivot = arr.get(0);

            __CLR4_4_100l47116a9.R.inc(8);List<Integer> less = new LinkedList<>();
            __CLR4_4_100l47116a9.R.inc(9);List<Integer> pivotList = new LinkedList<>();
            __CLR4_4_100l47116a9.R.inc(10);List<Integer> more = new LinkedList<>();

            // Partition
            __CLR4_4_100l47116a9.R.inc(11);for (Integer i: arr) {{
                __CLR4_4_100l47116a9.R.inc(12);if ((((i.compareTo(pivot) < 0)&&(__CLR4_4_100l47116a9.R.iget(13)!=0|true))||(__CLR4_4_100l47116a9.R.iget(14)==0&false)))
                    {__CLR4_4_100l47116a9.R.inc(15);less.add(i);
                }else {__CLR4_4_100l47116a9.R.inc(16);if ((((i.compareTo(pivot) > 0)&&(__CLR4_4_100l47116a9.R.iget(17)!=0|true))||(__CLR4_4_100l47116a9.R.iget(18)==0&false)))
                    {__CLR4_4_100l47116a9.R.inc(19);more.add(i);
                }else
                    {__CLR4_4_100l47116a9.R.inc(20);pivotList.add(i);
            }}}

            // Recursively sort sublists
            }__CLR4_4_100l47116a9.R.inc(21);less = quickSort(less);
            __CLR4_4_100l47116a9.R.inc(22);more = quickSort(more);

            // Concatenate results
            __CLR4_4_100l47116a9.R.inc(23);less.addAll(pivotList);
            __CLR4_4_100l47116a9.R.inc(24);less.addAll(more);
            __CLR4_4_100l47116a9.R.inc(25);return less;
        }
    }}finally{__CLR4_4_100l47116a9.R.flushNeeded();}}

}