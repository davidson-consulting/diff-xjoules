/* $$ This file has been instrumented by Clover 4.4.1#2019101123313948 $$ */package fr.davidson;

import org.junit.Test;

import static org.junit.Assert.assertTrue;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Random;

public class AppTest {static class __CLR4_4_1qql47116ba{public static com_atlassian_clover.CoverageRecorder R;public static com_atlassian_clover.CloverProfile[] profiles = { };@java.lang.SuppressWarnings("unchecked") public static <I, T extends I> I lambdaInc(final int i,final T l,final int si){java.lang.reflect.InvocationHandler h=new java.lang.reflect.InvocationHandler(){public java.lang.Object invoke(java.lang.Object p,java.lang.reflect.Method m,java.lang.Object[] a) throws Throwable{R.inc(i);R.inc(si);try{return m.invoke(l,a);}catch(java.lang.reflect.InvocationTargetException e){throw e.getCause()!=null?e.getCause():new RuntimeException("Clover failed to invoke instrumented lambda",e);}}};return (I)java.lang.reflect.Proxy.newProxyInstance(l.getClass().getClassLoader(),l.getClass().getInterfaces(),h);}static{com_atlassian_clover.CoverageRecorder _R=null;try{com_atlassian_clover.CloverVersionInfo.An_old_version_of_clover_is_on_your_compilation_classpath___Please_remove___Required_version_is___4_4_1();if(2019101123313948L!=com_atlassian_clover.CloverVersionInfo.getBuildStamp()){com_atlassian_clover.Clover.l("[CLOVER] WARNING: The Clover version used in instrumentation does not match the runtime version. You need to run instrumented classes against the same version of Clover that you instrumented with.");com_atlassian_clover.Clover.l("[CLOVER] WARNING: Instr=4.4.1#2019101123313948,Runtime="+com_atlassian_clover.CloverVersionInfo.getReleaseNum()+"#"+com_atlassian_clover.CloverVersionInfo.getBuildStamp());}R=com_atlassian_clover.Clover.getNullRecorder();_R=com_atlassian_clover.Clover.getNullRecorder();_R=com_atlassian_clover.Clover.getRecorder("\u002f\u0068\u006f\u006d\u0065\u002f\u0062\u0065\u006e\u006a\u0061\u006d\u0069\u006e\u002f\u0077\u006f\u0072\u006b\u0073\u0070\u0061\u0063\u0065\u002f\u0064\u0069\u0066\u0066\u002d\u0078\u006a\u006f\u0075\u006c\u0065\u0073\u002f\u0064\u0069\u0066\u0066\u002d\u006a\u006a\u006f\u0075\u006c\u0065\u0073\u002f\u0073\u0072\u0063\u002f\u0074\u0065\u0073\u0074\u002f\u0072\u0065\u0073\u006f\u0075\u0072\u0063\u0065\u0073\u002f\u0064\u0069\u0066\u0066\u002d\u006a\u006a\u006f\u0075\u006c\u0065\u0073\u002d\u0074\u006f\u0079\u002d\u006a\u0061\u0076\u0061\u002d\u0070\u0072\u006f\u006a\u0065\u0063\u0074\u002f\u0074\u0061\u0072\u0067\u0065\u0074\u002f\u0063\u006c\u006f\u0076\u0065\u0072\u002f\u0063\u006c\u006f\u0076\u0065\u0072\u002e\u0064\u0062",1654779573038L,8589935092L,67,profiles,new java.lang.String[]{"clover.distributed.coverage",null});}catch(java.lang.SecurityException e){java.lang.System.err.println("[CLOVER] FATAL ERROR: Clover could not be initialised because it has insufficient security privileges. Please consult the Clover documentation on the security policy file changes required. ("+e.getClass()+":"+e.getMessage()+")");}catch(java.lang.NoClassDefFoundError e){java.lang.System.err.println("[CLOVER] FATAL ERROR: Clover could not be initialised. Are you sure you have Clover in the runtime classpath? ("+e.getClass()+":"+e.getMessage()+")");}catch(java.lang.Throwable t){java.lang.System.err.println("[CLOVER] FATAL ERROR: Clover could not be initialised because of an unexpected error. ("+t.getClass()+":"+t.getMessage()+")");}R=_R;}}public static final com_atlassian_clover.TestNameSniffer __CLR4_4_1_TEST_NAME_SNIFFER=com_atlassian_clover.TestNameSniffer.NULL_INSTANCE;

    @Test
    public void testEmpty() {__CLR4_4_1qql47116ba.R.globalSliceStart(getClass().getName(),26);int $CLV_p$=0;java.lang.Throwable $CLV_t$=null;try{__CLR4_4_1y6g5z2q();$CLV_p$=1;}catch(java.lang.Throwable $CLV_t2$){if($CLV_p$==0&&$CLV_t$==null){$CLV_t$=$CLV_t2$;}__CLR4_4_1qql47116ba.R.rethrow($CLV_t2$);}finally{__CLR4_4_1qql47116ba.R.globalSliceEnd(getClass().getName(),"fr.davidson.AppTest.testEmpty",__CLR4_4_1_TEST_NAME_SNIFFER.getTestName(),26,$CLV_p$,$CLV_t$);}}private void  __CLR4_4_1y6g5z2q(){try{__CLR4_4_1qql47116ba.R.inc(26);
        __CLR4_4_1qql47116ba.R.inc(27);final List<Integer> emptyList = Collections.emptyList();
        __CLR4_4_1qql47116ba.R.inc(28);App.sort(emptyList);
        __CLR4_4_1qql47116ba.R.inc(29);assertTrue(emptyList.isEmpty());
    }finally{__CLR4_4_1qql47116ba.R.flushNeeded();}}

    @Test
    public void testSingleton() {__CLR4_4_1qql47116ba.R.globalSliceStart(getClass().getName(),30);int $CLV_p$=0;java.lang.Throwable $CLV_t$=null;try{__CLR4_4_1jmk8jku();$CLV_p$=1;}catch(java.lang.Throwable $CLV_t2$){if($CLV_p$==0&&$CLV_t$==null){$CLV_t$=$CLV_t2$;}__CLR4_4_1qql47116ba.R.rethrow($CLV_t2$);}finally{__CLR4_4_1qql47116ba.R.globalSliceEnd(getClass().getName(),"fr.davidson.AppTest.testSingleton",__CLR4_4_1_TEST_NAME_SNIFFER.getTestName(),30,$CLV_p$,$CLV_t$);}}private void  __CLR4_4_1jmk8jku(){try{__CLR4_4_1qql47116ba.R.inc(30);
        __CLR4_4_1qql47116ba.R.inc(31);final List<Integer> singletonList = Collections.singletonList(23);
        __CLR4_4_1qql47116ba.R.inc(32);App.sort(singletonList);
        __CLR4_4_1qql47116ba.R.inc(33);assertTrue(isSorted(singletonList));
    }finally{__CLR4_4_1qql47116ba.R.flushNeeded();}}

    @Test
    public void testRandom() {__CLR4_4_1qql47116ba.R.globalSliceStart(getClass().getName(),34);int $CLV_p$=0;java.lang.Throwable $CLV_t$=null;try{__CLR4_4_1ujoeasy();$CLV_p$=1;}catch(java.lang.Throwable $CLV_t2$){if($CLV_p$==0&&$CLV_t$==null){$CLV_t$=$CLV_t2$;}__CLR4_4_1qql47116ba.R.rethrow($CLV_t2$);}finally{__CLR4_4_1qql47116ba.R.globalSliceEnd(getClass().getName(),"fr.davidson.AppTest.testRandom",__CLR4_4_1_TEST_NAME_SNIFFER.getTestName(),34,$CLV_p$,$CLV_t$);}}private void  __CLR4_4_1ujoeasy(){try{__CLR4_4_1qql47116ba.R.inc(34);
        __CLR4_4_1qql47116ba.R.inc(35);final List<Integer> randomList = getRandomList(100);
        __CLR4_4_1qql47116ba.R.inc(36);App.sort(randomList);
        __CLR4_4_1qql47116ba.R.inc(37);assertTrue(isSorted(randomList));
    }finally{__CLR4_4_1qql47116ba.R.flushNeeded();}}

    @Test
    public void testRandomQuickSort() {__CLR4_4_1qql47116ba.R.globalSliceStart(getClass().getName(),38);int $CLV_p$=0;java.lang.Throwable $CLV_t$=null;try{__CLR4_4_1gaiw8z12();$CLV_p$=1;}catch(java.lang.Throwable $CLV_t2$){if($CLV_p$==0&&$CLV_t$==null){$CLV_t$=$CLV_t2$;}__CLR4_4_1qql47116ba.R.rethrow($CLV_t2$);}finally{__CLR4_4_1qql47116ba.R.globalSliceEnd(getClass().getName(),"fr.davidson.AppTest.testRandomQuickSort",__CLR4_4_1_TEST_NAME_SNIFFER.getTestName(),38,$CLV_p$,$CLV_t$);}}private void  __CLR4_4_1gaiw8z12(){try{__CLR4_4_1qql47116ba.R.inc(38);
        __CLR4_4_1qql47116ba.R.inc(39);final List<Integer> randomList = getRandomList(100);
        __CLR4_4_1qql47116ba.R.inc(40);final List<Integer> sortedRandomList = App.quickSort(randomList);
        __CLR4_4_1qql47116ba.R.inc(41);assertTrue(isSorted(sortedRandomList));
    }finally{__CLR4_4_1qql47116ba.R.flushNeeded();}}

    @Test
    public void testRandomQuickSortLarge() {__CLR4_4_1qql47116ba.R.globalSliceStart(getClass().getName(),42);int $CLV_p$=0;java.lang.Throwable $CLV_t$=null;try{__CLR4_4_1r1oq4416();$CLV_p$=1;}catch(java.lang.Throwable $CLV_t2$){if($CLV_p$==0&&$CLV_t$==null){$CLV_t$=$CLV_t2$;}__CLR4_4_1qql47116ba.R.rethrow($CLV_t2$);}finally{__CLR4_4_1qql47116ba.R.globalSliceEnd(getClass().getName(),"fr.davidson.AppTest.testRandomQuickSortLarge",__CLR4_4_1_TEST_NAME_SNIFFER.getTestName(),42,$CLV_p$,$CLV_t$);}}private void  __CLR4_4_1r1oq4416(){try{__CLR4_4_1qql47116ba.R.inc(42);
        __CLR4_4_1qql47116ba.R.inc(43);final List<Integer> randomList = getRandomList(100000);
        __CLR4_4_1qql47116ba.R.inc(44);final List<Integer> sortedRandomList = App.quickSort(randomList);
        __CLR4_4_1qql47116ba.R.inc(45);assertTrue(isSorted(sortedRandomList));
    }finally{__CLR4_4_1qql47116ba.R.flushNeeded();}}

    private List<Integer> getRandomList(final int bound) {try{__CLR4_4_1qql47116ba.R.inc(46);
        __CLR4_4_1qql47116ba.R.inc(47);final List<Integer> randomList = new ArrayList<>();
        __CLR4_4_1qql47116ba.R.inc(48);final Random random = new Random();
        __CLR4_4_1qql47116ba.R.inc(49);final int nbRandomElement = random.nextInt(bound);
        __CLR4_4_1qql47116ba.R.inc(50);for (int i = 0; (((i < nbRandomElement)&&(__CLR4_4_1qql47116ba.R.iget(51)!=0|true))||(__CLR4_4_1qql47116ba.R.iget(52)==0&false)); i++) {{
            __CLR4_4_1qql47116ba.R.inc(53);randomList.add(random.nextInt());
        }
        }__CLR4_4_1qql47116ba.R.inc(54);return randomList;
    }finally{__CLR4_4_1qql47116ba.R.flushNeeded();}}

    private static boolean isSorted(List<Integer> list) {try{__CLR4_4_1qql47116ba.R.inc(55);
        __CLR4_4_1qql47116ba.R.inc(56);int previous = list.get(0);
        __CLR4_4_1qql47116ba.R.inc(57);for (int i = 1; (((i < list.size())&&(__CLR4_4_1qql47116ba.R.iget(58)!=0|true))||(__CLR4_4_1qql47116ba.R.iget(59)==0&false)); i++) {{
            __CLR4_4_1qql47116ba.R.inc(60);int current = list.get(i);
            __CLR4_4_1qql47116ba.R.inc(61);if ((((current < previous)&&(__CLR4_4_1qql47116ba.R.iget(62)!=0|true))||(__CLR4_4_1qql47116ba.R.iget(63)==0&false))) {{
                __CLR4_4_1qql47116ba.R.inc(64);return false;
            }
            }__CLR4_4_1qql47116ba.R.inc(65);previous = current;
        }
        }__CLR4_4_1qql47116ba.R.inc(66);return true;
    }finally{__CLR4_4_1qql47116ba.R.flushNeeded();}}
}
